use crate::barracuda::mcts::Node;
use crate::barracuda::traits::{BarracudaBoard, BarracudaMove, BarracudaParams};

pub trait BarracudaUcbParams<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
    const DATA_REGISTERS: usize,
>: Send
{
    fn set_node(
        &mut self,
        state: &mut Node<Board, Params, Move, { DATA_REGISTERS }>,
        last_move: &Move,
    );
}

pub trait BarracudaSimulationParams<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
    const DATA_REGISTERS: usize,
>: Send
{
    fn set_node(
        &mut self,
        state: &mut Node<Board, Params, Move, { DATA_REGISTERS }>,
        last_move: &Move,
        eval: f32,
    );
}
