use crate::barracuda::traits::{BarracudaBoard, BarracudaMove, BarracudaParams, GameState, Player};

use crate::barracuda::b_core::components::backprop::Backprop;
use crate::barracuda::b_core::components::policy::Policy;
use crate::barracuda::b_core::components::simulate::Simulation;
use crate::barracuda::b_core::components::ucb::Ucb;
use crate::barracuda::b_core::params::{BarracudaSimulationParams, BarracudaUcbParams};
use std::sync::{Arc, Mutex, Weak};

pub struct BarracudaAlgorithms<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
    const DATA_REGISTERS: usize,
> {
    pub ucb_algorithms:
        Vec<Arc<Mutex<dyn BarracudaUcbParams<Board, Params, Move, { DATA_REGISTERS }>>>>,
    pub ucb: Ucb,
    pub policy: Arc<Mutex<dyn Policy<Board, Params, Move>>>,
    pub simulation_algorithms:
        Vec<Arc<Mutex<dyn BarracudaSimulationParams<Board, Params, Move, { DATA_REGISTERS }>>>>,
    pub simulation: Arc<Mutex<dyn Simulation<Board, Params, Move>>>,
    pub backprop_algorithms: Vec<Arc<Mutex<dyn Backprop<Board, Params, Move, { DATA_REGISTERS }>>>>,
}

#[derive(Debug, Clone)]
pub struct Node<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
    const DATA_REGISTERS: usize,
> {
    pub board: Board,
    pub moves: Vec<Move>,
    pub child_nodes: Vec<Arc<Mutex<Node<Board, Params, Move, { DATA_REGISTERS }>>>>,
    pub parent: Option<Weak<Mutex<Node<Board, Params, Move, { DATA_REGISTERS }>>>>,

    pub score: f32,
    pub visits: f32,

    pub registers: [f32; DATA_REGISTERS],
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > Node<Board, Params, Move, DATA_REGISTERS>
{
    pub fn new(
        board: Board,
        parent: Option<Weak<Mutex<Node<Board, Params, Move, { DATA_REGISTERS }>>>>,
    ) -> Self {
        Self {
            board,
            moves: vec![],
            child_nodes: vec![],
            parent,
            score: 0.0,
            visits: 1e-8,
            registers: [0.0; DATA_REGISTERS],
        }
    }

    pub fn eval(&self) -> f32 {
        self.score / self.visits
    }

    pub fn visits(&self) -> u32 {
        self.visits as u32
    }

    pub fn pv(node: Arc<Mutex<Node<Board, Params, Move, DATA_REGISTERS>>>) -> Vec<Move> {
        let mut moves = vec![];
        let mut current_node = node;
        while !current_node.lock().unwrap().moves.is_empty() {
            let mut highest_visits = -1f32;
            let mut best_move = None;
            let current_node_lock = current_node.lock().unwrap();
            let child_nodes = current_node_lock.child_nodes.clone();
            let child_moves = current_node_lock.moves.clone();
            drop(current_node_lock);

            for (child, mv) in child_nodes.into_iter().zip(child_moves.into_iter()) {
                let child_borrow = child.lock().unwrap();
                if child_borrow.visits > highest_visits {
                    highest_visits = child_borrow.visits;
                    current_node = child.clone();
                    best_move = Some(mv);
                }
            }
            if let Some(best_move) = best_move {
                moves.push(best_move)
            }
        }
        moves
    }

    pub fn search(
        node: Arc<Mutex<Node<Board, Params, Move, { DATA_REGISTERS }>>>,
        algo: &BarracudaAlgorithms<Board, Params, Move, { DATA_REGISTERS }>,
    ) {
        //Select
        let mut nodes = vec![];
        let mut current_node = node;
        nodes.push(current_node.clone());
        while !current_node.lock().unwrap().moves.is_empty() {
            let mut highest_ucb = f32::NEG_INFINITY;

            let current_node_lock = current_node.lock().unwrap();
            let child_nodes = current_node_lock.child_nodes.clone();
            let moves = current_node_lock.moves.clone();
            drop(current_node_lock);
            let mut selected_node = None;
            for (child, mv) in child_nodes.into_iter().zip(moves.into_iter()) {
                let mut child_lock = child.lock().unwrap();
                for algorithm in &algo.ucb_algorithms {
                    algorithm.lock().unwrap().set_node(&mut child_lock, &mv);
                }
                let ucb = algo.ucb.ucb();
                if ucb > highest_ucb || selected_node.is_none() {
                    highest_ucb = ucb;
                    selected_node = Some(child.clone());
                }
            }
            if let Some(selected_node) = selected_node {
                current_node = selected_node;
                nodes.push(current_node.clone());
            } else {
                println!("#WARNING: NO SELECTIONS OCCURRED")
            }
        }
        //Expand
        let mut rollout_score;
        {
            let mut node = current_node.lock().unwrap();
            let moves = node.board.get_moves();
            let game_state = node.board.game_state();
            if moves.is_empty() || !matches!(game_state, GameState::Ongoing) {
                rollout_score = match game_state {
                    GameState::Ongoing => {
                        panic!()
                    }
                    GameState::End(color) => {
                        if let Some(color) = color {
                            match color {
                                Player::P1 => 1f32,
                                Player::P2 => 0f32,
                            }
                        } else {
                            0.5f32
                        }
                    }
                };
                if node.board.turn() == Player::P1 {
                    rollout_score = 1f32 - rollout_score;
                }
            } else {
                let mut child_nodes = vec![];
                for mv in &moves {
                    let mut new_board = node.board.clone();
                    new_board.make_move(*mv);
                    let child_node =
                        Node::new(new_board, Some(Arc::downgrade(&current_node.clone())));
                    child_nodes.push(Arc::new(Mutex::new(child_node)));
                }

                let index = algo.policy.lock().unwrap().pick(&node.board, &moves);
                let move_made = moves[index];
                let child_node = child_nodes[index].clone();
                node.child_nodes = child_nodes;
                node.moves = moves;
                drop(node);
                {
                    let child_node = &mut child_node.lock().unwrap();
                    let rollout = algo.simulation.lock().unwrap().simulate(&child_node.board);
                    rollout_score = match child_node.board.turn() {
                        Player::P1 => rollout,
                        Player::P2 => 1f32 - rollout,
                    };
                    for algorithm in &algo.simulation_algorithms {
                        algorithm
                            .lock()
                            .unwrap()
                            .set_node(child_node, &move_made, rollout_score)
                    }
                }
                nodes.push(child_node);
            }
        }
        let mut last_score = rollout_score;
        for node in nodes.iter().rev() {
            let mut node = node.lock().unwrap();
            for algorithm in &algo.backprop_algorithms {
                algorithm.lock().unwrap().backprop(&mut node, last_score);
            }
            node.score += last_score;
            node.visits += 1f32;
            last_score = 1f32 - last_score;
        }
    }
}
