use crate::barracuda::b_core::components::policy::Policy;
use crate::barracuda::b_core::components::simulate::Simulation;
use crate::barracuda::traits::GameState;
use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams, Player,
};
use std::sync::{Arc, Mutex};

pub struct RandomPlayout<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
> {
    depth: usize,
    policy: Arc<Mutex<dyn Policy<Board, Params, Move>>>,
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    BarracudaAlgorithm for RandomPlayout<Board, Params, Move>
{
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    RandomPlayout<Board, Params, Move>
{
    pub fn new(depth: usize, policy: Arc<Mutex<dyn Policy<Board, Params, Move>>>) -> Self {
        Self { depth, policy }
    }

    pub fn eval(&mut self, board: &Board) -> f32 {
        match board.game_state() {
            GameState::Ongoing => 0.5,
            GameState::End(winner) => match winner {
                None => 0.5,
                Some(color) => match color {
                    Player::P1 => 1.0,
                    Player::P2 => 0.0,
                },
            },
        }
    }
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    Simulation<Board, Params, Move> for RandomPlayout<Board, Params, Move>
{
    fn simulate(&mut self, board: &Board) -> f32 {
        let mut depth = 0usize;
        let mut board = board.clone();
        loop {
            if depth >= self.depth {
                break;
            }
            let game_state = board.game_state();
            if !matches!(game_state, GameState::Ongoing) {
                break;
            }
            let moves = board.get_moves();
            if moves.is_empty() {
                break;
            }
            let index = self.policy.lock().unwrap().pick(&board, &moves);
            board.make_move(moves[index]);
            depth += 1;
        }
        self.eval(&board)
    }
}
