use crate::barracuda::b_core::components::static_eval::StaticEval;
use crate::barracuda::traits::{BarracudaAlgorithm, BarracudaBoard, Player};
use crate::cf::cf::{CfParams, ConnectFour};

pub struct CfEval {}

impl BarracudaAlgorithm for CfEval {}

impl StaticEval<ConnectFour, CfParams, usize> for CfEval {
    fn evaluate(&mut self, board: &ConnectFour) -> f32 {
        let center = board.board().len() as f32 * 0.5f32;
        let mut eval = 0f32;
        for (index, col) in board.board().iter().enumerate() {
            let index = index as f32;
            let col_score = 1f32 / (1f32 + (index - center).powi(2));
            for p in col {
                if let Some(p) = p {
                    match p {
                        Player::P1 => {
                            eval += col_score;
                        }
                        Player::P2 => {
                            eval -= col_score;
                        }
                    }
                }
            }
        }
        match board.turn() {
            Player::P1 => eval,
            Player::P2 => -eval,
        }
    }
}
