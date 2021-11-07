use crate::barracuda::debugger::Debugger;
use crate::barracuda::mcts::{BarracudaAlgorithms, Node};
use crate::barracuda::traits::{BarracudaBoard, BarracudaMove, BarracudaParams};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct BarracudaRunner<
    Board: BarracudaBoard<Params, Move>,
    Params: BarracudaParams,
    Move: BarracudaMove,
    const DATA_REGISTERS: usize,
> {
    root: Arc<Mutex<Node<Board, Params, Move, DATA_REGISTERS>>>,
    algorithms: BarracudaAlgorithms<Board, Params, Move, { DATA_REGISTERS }>,
    board: Board,
    params: PhantomData<Params>,
}

impl<
        Board: BarracudaBoard<Params, Move>,
        Params: BarracudaParams,
        Move: BarracudaMove,
        const DATA_REGISTERS: usize,
    > BarracudaRunner<Board, Params, Move, DATA_REGISTERS>
{
    pub fn new(
        algorithms: BarracudaAlgorithms<Board, Params, Move, { DATA_REGISTERS }>,
        params: Params,
    ) -> Self {
        let board = Board::new(params);
        Self {
            root: Arc::new(Mutex::new(Node::new(board.clone(), None))),
            algorithms,
            board,
            params: PhantomData::default(),
        }
    }

    pub fn search<D: Debugger<Board, Params, Move>>(&mut self, think_time: f32) {
        let time = Instant::now();
        let mut debug_counter = D::sampling_rate();
        while time.elapsed().as_secs_f32() < think_time {
            Node::search(self.root.clone(), &self.algorithms);
            if time.elapsed() > debug_counter {
                let pv = Node::pv(self.root.clone());
                let root = self.root.lock().unwrap();
                D::debug(
                    &self.board,
                    root.eval(),
                    root.visits(),
                    pv.len() as u32,
                    &pv,
                );
                debug_counter = time.elapsed() + D::sampling_rate();
            }
        }
    }

    pub fn make_move(&mut self, mv: Move) {
        self.board.make_move(mv);
        self.root = Arc::new(Mutex::new(Node::new(self.board.clone(), None)));
    }

    pub fn best_move(&self) -> Move {
        Node::pv(self.root.clone())[0]
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = board.clone();
        self.root = Arc::new(Mutex::new(Node::new(board, None)));
    }
}
