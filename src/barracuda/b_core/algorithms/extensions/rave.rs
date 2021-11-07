use crate::barracuda::b_core::params::{BarracudaSimulationParams, BarracudaUcbParams};
use crate::barracuda::mcts::Node;
use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams,
};

use crate::barracuda::b_core::algorithms::ucb::formula::Formula;
use std::marker::PhantomData;

pub struct Rave<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove> {
    eval: f32,
    score_register: usize,
    visit_register: usize,
    p: PhantomData<(Board, Params, Move)>,
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    BarracudaAlgorithm for Rave<Board, Params, Move>
{
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    Rave<Board, Params, Move>
{
    pub fn new(score_register: usize, visit_register: usize) -> Self {
        Self {
            eval: 0f32,
            score_register,
            visit_register,
            p: PhantomData::default(),
        }
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaUcbParams<Board, Params, Move, { DATA_REGISTERS }> for Rave<Board, Params, Move>
{
    fn set_node(&mut self, state: &mut Node<Board, Params, Move, { DATA_REGISTERS }>, _: &Move) {
        self.eval =
            state.registers[self.score_register] / (state.registers[self.visit_register] + 1e-8);
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaSimulationParams<Board, Params, Move, { DATA_REGISTERS }>
    for Rave<Board, Params, Move>
{
    fn set_node(
        &mut self,
        state: &mut Node<Board, Params, Move, { DATA_REGISTERS }>,
        last_move: &Move,
        eval: f32,
    ) {
        state.registers[self.score_register] += eval;
        state.registers[self.visit_register] += 1f32;

        let mut skips = 0u16;
        if let Some(mut parent) = state.parent.clone() {
            loop {
                if let Some(u_parent) = parent.upgrade() {
                    skips += 1;
                    let mut parent_lock = u_parent.lock().unwrap();
                    if skips > 2 && skips % 2 == 0 {
                        let mut found = false;
                        for mv in parent_lock.moves.iter() {
                            if *mv == *last_move {
                                found = true;
                                break;
                            }
                        }
                        if found {
                            parent_lock.registers[self.score_register] += eval;
                            parent_lock.registers[self.visit_register] += 1f32;
                        }
                    }
                    let node_parent = parent_lock.parent.clone();
                    drop(parent_lock);
                    if let Some(node_parent) = node_parent {
                        parent = node_parent;
                    } else {
                        break;
                    }
                } else {
                    println!("WARNING: Parent Node has been dropped")
                }
            }
        }
    }
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove> Formula
    for Rave<Board, Params, Move>
{
    fn get(&self) -> f32 {
        self.eval
    }
}
