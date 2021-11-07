use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams,
};

pub trait Policy<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>:
    BarracudaAlgorithm
{
    fn pick(&mut self, board: &Board, moves: &[Move]) -> usize;
}
