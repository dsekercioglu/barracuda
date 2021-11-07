use crate::barracuda::b_runner::BarracudaRunner;
use crate::cf::cf::{CfParams, ConnectFour};
use crate::cli::cli::CommandOut::{Error, Success, Warning};
use crate::tictactoe::ttt::{Square, TicTacToeBoard, TicTacToeParams};

use crate::barracuda::b_core::algorithms::policy::uniform::Uniform;
use crate::barracuda::b_core::algorithms::simulate::random_playout::RandomPlayout;
use crate::barracuda::b_core::algorithms::ucb::exploration::Exploration;
use crate::barracuda::b_core::algorithms::ucb::formula::Add;
use crate::barracuda::b_core::algorithms::ucb::value::Value;
use crate::barracuda::b_core::components::ucb::Ucb;
use crate::barracuda::debugger::BarracudaDebug;
use crate::barracuda::mcts::BarracudaAlgorithms;
use crate::barracuda::traits::{BarracudaBoard, BarracudaMove, BarracudaParams};
use crate::cli::cli::Runner::{Cf, Ttt};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

enum Runner {
    Cf(Arc<Mutex<BarracudaRunner<ConnectFour, CfParams, usize, 0>>>),
    Ttt(Arc<Mutex<BarracudaRunner<TicTacToeBoard, TicTacToeParams, Square, 0>>>),
    Deactivated,
}

pub enum CommandOut<T, U, V> {
    Success(T),
    Warning((T, U)),
    Error(V),
}

enum Cmd<'a> {
    Analyze(Vec<&'a str>),
    Mode(Vec<&'a str>),
    MakeMove(Vec<&'a str>),
}

pub struct Cli {
    runner: Runner,
    current: Option<JoinHandle<()>>,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            runner: Runner::Deactivated,
            current: None,
        }
    }

    pub fn parse(&mut self, input: String) -> CommandOut<bool, String, String> {
        match self.current.take() {
            None => {}
            Some(handle) => {
                handle.join().unwrap();
            }
        }
        let input = input.split_ascii_whitespace().collect::<Vec<_>>();
        if input.is_empty() {
            return Error("No commands have been given".to_string());
        }
        let cmd = input[0];
        let cmd = match cmd {
            "analyze" => Cmd::Analyze(input[1..].to_vec()),
            "mode" => {
                if input.len() > 1 {
                    Cmd::Mode(input[1..].to_vec())
                } else {
                    return Error("No parameters have been given to Mode".to_string());
                }
            }
            "move" => {
                if input.len() > 1 {
                    Cmd::MakeMove(input[1..].to_vec())
                } else {
                    return Error("No parameters have been given to Move".to_string());
                }
            }
            _ => {
                return Error("Unrecognized command".to_string());
            }
        };

        match cmd {
            Cmd::Analyze(params) => {
                if params.is_empty() {
                    return Error("No parameters have been given to command 'analyze'".to_string());
                }
                let think_time = match params[0].parse::<f32>() {
                    Ok(time) => time,
                    Err(error) => {
                        return Error(error.to_string());
                    }
                };
                match &mut self.runner {
                    Runner::Cf(cf_runner) => {
                        let runner = cf_runner.clone();
                        self.current = Some(std::thread::spawn(move || {
                            runner.lock().unwrap().search::<BarracudaDebug>(think_time);
                            println!("{:?}", runner.lock().unwrap().best_move());
                        }));
                    }
                    Runner::Ttt(ttt_runner) => {
                        let runner = ttt_runner.clone();
                        self.current = Some(std::thread::spawn(move || {
                            runner.lock().unwrap().search::<BarracudaDebug>(think_time);
                            println!("{:?}", runner.lock().unwrap().best_move());
                        }));
                    }
                    Runner::Deactivated => {
                        return Error("No game has been activated".to_string());
                    }
                }
                return if params.len() > 1 {
                    Warning((
                        true,
                        "More than 1 parameter has been given, the rest have been ignored"
                            .to_string(),
                    ))
                } else {
                    Success(true)
                };
            }
            Cmd::Mode(mode) => match mode[0] {
                "cf" => {
                    self.runner = Cf(Arc::new(Mutex::new(Self::new_runner::<
                        ConnectFour,
                        CfParams,
                        usize,
                    >())));
                }
                "ttt" => {
                    self.runner = Ttt(Arc::new(Mutex::new(Self::new_runner::<
                        TicTacToeBoard,
                        TicTacToeParams,
                        Square,
                    >())));
                }
                _ => {
                    return Error("Unrecognized game".to_string());
                }
            },
            Cmd::MakeMove(params) => {
                match &self.runner {
                    Cf(_runner) => {
                        //runner.clone().lock().unwrap().make_move(params[0])
                    }
                    Ttt(_runner) => {}
                    Runner::Deactivated => {}
                }
                return if params.len() > 1 {
                    Warning((
                        true,
                        "More than 1 parameter has been given, the rest have been ignored"
                            .to_string(),
                    ))
                } else {
                    Success(true)
                };
            }
        }

        Success(true)
    }

    fn new_runner<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
    >() -> BarracudaRunner<Board, Params, Move, 0> {
        let value = Arc::new(Mutex::new(Value::new()));
        let exploration = Arc::new(Mutex::new(Exploration::new(1.414)));

        let runner = BarracudaRunner::<Board, Params, Move, 0>::new(
            BarracudaAlgorithms {
                ucb_algorithms: vec![value.clone(), exploration.clone()],
                ucb: Ucb::new(Arc::new(Mutex::new(Add::new(vec![
                    value.clone(),
                    exploration.clone(),
                ])))),
                policy: Arc::new(Mutex::new(Uniform::new())),
                simulation_algorithms: vec![],
                simulation: Arc::new(Mutex::new(RandomPlayout::new(
                    usize::MAX,
                    Arc::new(Mutex::new(Uniform::new())),
                ))),
                backprop_algorithms: vec![],
            },
            Params::default(),
        );
        runner
    }
}
