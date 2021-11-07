use crate::barracuda::b_core::params::BarracudaUcbParams;
use crate::barracuda::traits::{BarracudaBoard, BarracudaMove, BarracudaParams};

use crate::barracuda::mcts::Node;

use crate::barracuda::b_core::algorithms::ucb::formula::Formula;

pub struct Value {
    win: f32,
}

impl Value {
    pub fn new() -> Self {
        Self { win: 0f32 }
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaUcbParams<Board, Params, Move, { DATA_REGISTERS }> for Value
{
    fn set_node(&mut self, state: &mut Node<Board, Params, Move, DATA_REGISTERS>, _: &Move) {
        self.win = 1f32 - state.score / state.visits
    }
}

impl Formula for Value {
    fn get(&self) -> f32 {
        self.win
    }
}
