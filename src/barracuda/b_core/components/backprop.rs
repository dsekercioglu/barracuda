use crate::barracuda::mcts::Node;
use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams,
};

pub trait Backprop<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
    const DATA_REGISTERS: usize,
>: BarracudaAlgorithm
{
    fn backprop(&mut self, node: &mut Node<Board, Params, Move, { DATA_REGISTERS }>, score: f32);
}
