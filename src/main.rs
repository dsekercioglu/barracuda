use crate::barracuda::b_core::algorithms::extensions::mast::Mast;
use crate::barracuda::b_core::algorithms::extensions::rave::Rave;
use crate::barracuda::b_core::algorithms::extensions::t_table::TranspositionTable;
use crate::barracuda::b_core::algorithms::policy::uniform::Uniform;
use crate::barracuda::b_core::algorithms::simulate::random_playout::RandomPlayout;
use crate::barracuda::b_core::algorithms::ucb::exploration::Exploration;
use crate::barracuda::b_core::algorithms::ucb::formula::{Add, Const, Mul};
use crate::barracuda::b_core::algorithms::ucb::random::Random;
use crate::barracuda::b_core::algorithms::ucb::value::Value;
use crate::barracuda::b_core::components::ucb::Ucb;
use crate::barracuda::b_runner::BarracudaRunner;
use crate::barracuda::debugger::BarracudaDebug;
use crate::barracuda::mcts::BarracudaAlgorithms;
use crate::barracuda::traits::BarracudaBoard;

use crate::cf::cf::{CfParams, ConnectFour};
use crate::cli::cli::{Cli, CommandOut};
use crate::tictactoe::ttt::{Square, TicTacToeBoard, TicTacToeParams};

use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use text_io::read;

mod barracuda;
mod cf;
mod cli;
mod tictactoe;

/*
TODO: Add a proper CLI
TODO: Parallelize Search
TODO: Work on UCB Expression Tree
TODO: Independent Barracuda GUI (Made in any language)
TODO: Independent Rust Matrix Lib (Should be able to run on most GPUs and fallback on CPU when necessary)
TODO: Modular Alpha Beta Search System
TODO: Implement more Barracuda-Compatible Board Games
 */

fn main() {
    let value = Arc::new(Mutex::new(Value::new()));
    let exploration = Arc::new(Mutex::new(Exploration::new(1f32)));
    let mast = Arc::new(Mutex::new(Mast::new(0.05)));
    let rave = Arc::new(Mutex::new(Rave::new(0, 1)));
    let random = Arc::new(Mutex::new(Random::new(1e-3)));
    let t_table = Arc::new(Mutex::new(TranspositionTable::new()));

    let mut br_all_features = BarracudaRunner::<TicTacToeBoard, TicTacToeParams, Square, 4>::new(
        BarracudaAlgorithms {
            ucb_algorithms: vec![
                value.clone(),
                exploration.clone(),
                mast.clone(),
                rave.clone(),
            ],
            ucb: Ucb::new(Arc::new(Mutex::new(Add::new(vec![
                random,
                t_table.clone(),
                value,
                exploration,
                mast.clone(),
                Arc::new(Mutex::new(Mul::new(vec![rave.clone(), Const::new(0.1f32)]))),
            ])))),
            policy: Arc::new(Mutex::new(Uniform::new())),
            simulation_algorithms: vec![mast, rave],
            simulation: Arc::new(Mutex::new(RandomPlayout::new(
                usize::MAX,
                Arc::new(Mutex::new(Uniform::new())),
            ))),
            backprop_algorithms: vec![t_table.clone()],
        },
        TicTacToeParams,
    );

    let mut board = TicTacToeBoard::new(TicTacToeParams);

    for _ in 0..42 {
        br_all_features.search::<BarracudaDebug>(10f32);
        t_table.lock().unwrap().clear();
        let mv = br_all_features.best_move();
        println!("move: {:?}", mv);
        board.make_move(mv);
        br_all_features.make_move(mv);
    }
}
