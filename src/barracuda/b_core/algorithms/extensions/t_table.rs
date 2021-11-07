use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams,
};
use std::collections::HashMap;

use crate::barracuda::b_core::algorithms::ucb::formula::Formula;
use crate::barracuda::b_core::components::backprop::Backprop;
use crate::barracuda::b_core::params::BarracudaUcbParams;
use crate::barracuda::mcts::Node;
use std::marker::PhantomData;

pub struct TranspositionTable<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
    const DATA_REGISTERS: usize,
> {
    table: HashMap<Board, (f32, f32)>,
    eval: f32,
    p: PhantomData<(Params, Move)>,
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaAlgorithm for TranspositionTable<Board, Params, Move, { DATA_REGISTERS }>
{
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > TranspositionTable<Board, Params, Move, { DATA_REGISTERS }>
{
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            eval: 0f32,
            p: PhantomData::default(),
        }
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > Backprop<Board, Params, Move, { DATA_REGISTERS }>
    for TranspositionTable<Board, Params, Move, { DATA_REGISTERS }>
{
    fn backprop(&mut self, node: &mut Node<Board, Params, Move, { DATA_REGISTERS }>, score: f32) {
        if let Some(entry) = self.table.get_mut(&node.board) {
            entry.0 += score;
            entry.1 += 1f32;
        }
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaUcbParams<Board, Params, Move, DATA_REGISTERS>
    for TranspositionTable<Board, Params, Move, { DATA_REGISTERS }>
{
    fn set_node(&mut self, state: &mut Node<Board, Params, Move, DATA_REGISTERS>, _: &Move) {
        let eval = if let Some((score, visits)) = self.table.get(&state.board) {
            1f32 - *score / *visits
        } else {
            0f32
        };
        self.eval = eval;
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > Formula for TranspositionTable<Board, Params, Move, { DATA_REGISTERS }>
{
    fn get(&self) -> f32 {
        self.eval
    }
}
