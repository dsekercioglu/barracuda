use crate::barracuda::b_core::components::simulate::Simulation;
use crate::barracuda::b_core::components::static_eval::StaticEval;
use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams,
};
use std::sync::{Arc, Mutex};

pub struct Evaluate<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
> {
    static_eval: Arc<Mutex<dyn StaticEval<Board, Params, Move>>>,
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    BarracudaAlgorithm for Evaluate<Board, Params, Move>
{
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    Evaluate<Board, Params, Move>
{
    pub fn new(static_eval: Arc<Mutex<dyn StaticEval<Board, Params, Move>>>) -> Self {
        Self { static_eval }
    }
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    Simulation<Board, Params, Move> for Evaluate<Board, Params, Move>
{
    fn simulate(&mut self, board: &Board) -> f32 {
        self.static_eval.lock().unwrap().evaluate(board)
    }
}
