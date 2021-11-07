use crate::barracuda::b_core::components::policy::Policy;
use crate::barracuda::b_core::components::static_eval::StaticEval;
use crate::barracuda::traits::{
    BarracudaAlgorithm, BarracudaBoard, BarracudaMove, BarracudaParams,
};
use rand::distributions::Uniform;
use rand::Rng;
use std::sync::{Arc, Mutex};

pub struct Softmax<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
> {
    eval: Arc<Mutex<dyn StaticEval<Board, Params, Move>>>,
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    BarracudaAlgorithm for Softmax<Board, Params, Move>
{
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    Softmax<Board, Params, Move>
{
    pub fn new(static_eval: Arc<Mutex<dyn StaticEval<Board, Params, Move>>>) -> Self {
        Self { eval: static_eval }
    }
}

impl<Board: BarracudaBoard<Params, Move>, Params: BarracudaParams, Move: BarracudaMove>
    Policy<Board, Params, Move> for Softmax<Board, Params, Move>
{
    fn pick(&mut self, board: &Board, moves: &[Move]) -> usize {
        let mut move_scores = vec![];
        let mut min = f32::INFINITY;

        let mut eval = self.eval.lock().unwrap();
        for mv in moves {
            let mut board = board.clone();
            board.make_move(*mv);
            let score = eval.evaluate(&board);
            move_scores.push(score);
            min = min.min(score);
        }
        let mut sum = 0f32;
        for score in &mut move_scores {
            *score = (*score - min).exp();
            sum += *score;
        }
        for score in &mut move_scores {
            *score /= sum;
        }
        let distr = Uniform::new(0f32, 1f32);
        let random = rand::thread_rng().sample(distr);
        let mut index = 0;
        while index < move_scores.len() - 1 && random > move_scores[index] {
            index += 1;
        }
        index
    }
}
