use crate::barracuda::b_core::algorithms::ucb::formula::Formula;
use crate::barracuda::b_core::components::policy::Policy;
use crate::barracuda::b_core::params::{BarracudaSimulationParams, BarracudaUcbParams};
use crate::barracuda::mcts::Node;
use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams, Player,
};
use rand::distributions::Uniform;
use rand::Rng;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Mutex;

pub struct Mast<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove> {
    move_map: Mutex<MoveMap<Move>>,
    eval: Mutex<f32>,
    p: PhantomData<(Board, Params)>,
}
impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    BarracudaAlgorithm for Mast<Board, Params, Move>
{
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    Mast<Board, Params, Move>
{
    pub fn new(rolling_avg: f32) -> Self {
        Self {
            move_map: Mutex::new(MoveMap::new(rolling_avg)),
            eval: Mutex::new(0f32),
            p: PhantomData::default(),
        }
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaUcbParams<Board, Params, Move, { DATA_REGISTERS }> for Mast<Board, Params, Move>
{
    fn set_node(
        &mut self,
        state: &mut Node<Board, Params, Move, { DATA_REGISTERS }>,
        last_move: &Move,
    ) {
        let move_map_score = self
            .move_map
            .lock()
            .unwrap()
            .get(state.board.turn(), *last_move);
        let visits = state.visits;
        *self.eval.lock().unwrap() = move_map_score.unwrap_or(0f32) / visits;
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaSimulationParams<Board, Params, Move, { DATA_REGISTERS }>
    for Mast<Board, Params, Move>
{
    fn set_node(
        &mut self,
        state: &mut Node<Board, Params, Move, { DATA_REGISTERS }>,
        last_move: &Move,
        eval: f32,
    ) {
        self.move_map
            .lock()
            .unwrap()
            .add(state.board.turn(), *last_move, eval);
    }
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    Policy<Board, Params, Move> for Mast<Board, Params, Move>
{
    fn pick(&mut self, board: &Board, moves: &[Move]) -> usize {
        let move_map = self.move_map.lock().unwrap();
        let mut move_scores = vec![];
        let mut min = f32::INFINITY;
        for mv in moves {
            let score = move_map.get(board.turn(), *mv).unwrap_or(0f32);
            move_scores.push(score);
            min = min.min(score);
        }
        let mut sum = 0f32;
        for score in &mut move_scores {
            *score = (*score - min).exp();
            sum += *score;
        }
        for score in &mut move_scores {
            *score /= sum;
        }
        let distr = Uniform::new(0f32, 1f32);
        let random = rand::thread_rng().sample(distr);
        let mut index = 0;
        while index < move_scores.len() - 1 && random > move_scores[index] {
            index += 1;
        }
        index
    }
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove> Formula
    for Mast<Board, Params, Move>
{
    fn get(&self) -> f32 {
        *self.eval.lock().unwrap()
    }
}

pub struct MoveMap<Move: BarracudaMove> {
    move_map: HashMap<(Player, Move), f32>,
    rolling_avg: f32,
}

impl<Move: BarracudaMove> MoveMap<Move> {
    pub fn new(rolling_avg: f32) -> Self {
        Self {
            move_map: HashMap::new(),
            rolling_avg,
        }
    }

    pub fn all_moves(&self) -> Vec<((Player, Move), f32)> {
        self.move_map
            .iter()
            .map(|(mv, score)| (*mv, *score))
            .collect::<Vec<_>>()
    }

    pub fn add(&mut self, color: Player, mv: Move, score: f32) {
        if let Some(prev_score) = self.move_map.get_mut(&(color, mv)) {
            *prev_score = *prev_score * (1f32 - self.rolling_avg) + score * self.rolling_avg;
        } else {
            self.move_map.insert((color, mv), score);
        }
    }

    pub fn get(&self, color: Player, mv: Move) -> Option<f32> {
        self.move_map.get(&(color, mv)).copied()
    }
}
