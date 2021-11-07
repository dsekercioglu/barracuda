use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams,
};

pub trait Simulation<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
>: BarracudaAlgorithm
{
    fn simulate(&mut self, board: &Board) -> f32;
}
