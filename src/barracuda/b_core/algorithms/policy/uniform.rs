use crate::barracuda::b_core::components::policy::Policy;
use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams,
};
use rand::Rng;

pub struct Uniform;

impl Uniform {
    pub fn new() -> Self {
        Self {}
    }
}

impl BarracudaAlgorithm for Uniform {}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    Policy<Board, Params, Move> for Uniform
{
    fn pick(&mut self, _: &Board, moves: &[Move]) -> usize {
        rand::thread_rng().gen_range(0..moves.len())
    }
}
