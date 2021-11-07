use crate::barracuda::b_core::params::BarracudaUcbParams;
use crate::barracuda::traits::{BarracudaBoard, BarracudaMove, BarracudaParams};

use crate::barracuda::mcts::Node;

use crate::barracuda::b_core::algorithms::ucb::formula::Formula;

pub struct Exploration {
    temperature: f32,
    exploration: f32,
}

impl Exploration {
    pub fn new(temperature: f32) -> Self {
        Self {
            exploration: 0f32,
            temperature,
        }
    }
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaUcbParams<Board, Params, Move, { DATA_REGISTERS }> for Exploration
{
    fn set_node(&mut self, state: &mut Node<Board, Params, Move, { DATA_REGISTERS }>, _: &Move) {
        let parent_visits = if let Some(ref parent) = state.parent {
            if let Some(strong) = parent.upgrade() {
                strong.lock().unwrap().visits
            } else {
                println!("# WARNING PARENT NODE HAS BEEN DROPPED");
                0f32
            }
        } else {
            0f32
        };
        self.exploration = self.temperature * (parent_visits.ln() / state.visits).sqrt();
    }
}

impl Formula for Exploration {
    fn get(&self) -> f32 {
        self.exploration
    }
}
