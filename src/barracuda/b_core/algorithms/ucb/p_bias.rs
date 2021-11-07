use crate::barracuda::b_core::params::BarracudaUcbParams;
use crate::barracuda::traits::{BarracudaBoard, BarracudaMove, BarracudaParams};

use crate::barracuda::b_core::algorithms::ucb::formula::Formula;
use crate::barracuda::b_core::components::static_eval::StaticEval;
use crate::barracuda::mcts::Node;
use std::sync::{Arc, Mutex};

pub struct ProgressiveBias<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
    const DATA_REGISTERS: usize,
> {
    flag_register: usize,
    evaluation_register: usize,
    static_eval: Arc<Mutex<dyn StaticEval<Board, Params, Move>>>,
    eval: f32,
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > ProgressiveBias<Board, Params, Move, { DATA_REGISTERS }>
{
    pub fn new(
        flag_register: usize,
        evaluation_register: usize,
        static_eval: Arc<Mutex<dyn StaticEval<Board, Params, Move>>>,
    ) -> Self {
        Self {
            flag_register,
            evaluation_register,
            static_eval,
            eval: 0f32,
        }
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaUcbParams<Board, Params, Move, { DATA_REGISTERS }>
    for ProgressiveBias<Board, Params, Move, { DATA_REGISTERS }>
{
    fn set_node(&mut self, state: &mut Node<Board, Params, Move, { DATA_REGISTERS }>, _: &Move) {
        let mut eval = self.static_eval.lock().unwrap();
        if state.registers[self.flag_register] <= 0.01f32 {
            state.registers[self.flag_register] = 1f32;
            state.registers[self.evaluation_register] = eval.evaluate(&state.board)
        }
        self.eval = 1f32 - state.registers[self.evaluation_register] / state.visits;
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > Formula for ProgressiveBias<Board, Params, Move, { DATA_REGISTERS }>
{
    fn get(&self) -> f32 {
        self.eval
    }
}
