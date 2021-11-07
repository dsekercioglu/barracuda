use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams,
};

pub trait StaticEval<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
>: BarracudaAlgorithm
{
    fn evaluate(&mut self, board: &Board) -> f32;
}
