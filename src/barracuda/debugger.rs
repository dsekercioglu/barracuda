use crate::barracuda::traits::BarracudaBoard;
use std::time::Duration;

pub trait Debugger<Board: Clone + BarracudaBoard<Params, Move>, Params: Send, Move: Copy> {
    fn new() -> Self;

    fn sampling_rate() -> Duration;

    fn debug(board: &Board, eval: f32, nodes: u32, depth: u32, pv: &[Move]);
}

pub struct NoDebug;

impl<Board: Clone + BarracudaBoard<Params, Move>, Params: Send, Move: Copy>
    Debugger<Board, Params, Move> for NoDebug
{
    fn new() -> Self {
        Self {}
    }

    fn sampling_rate() -> Duration {
        Duration::from_secs_f32(f32::INFINITY)
    }

    fn debug(_: &Board, _: f32, _: u32, _: u32, _: &[Move]) {}
}

pub struct BarracudaDebug;

impl<Board: Clone + BarracudaBoard<Params, Move>, Params: Send, Move: Copy>
    Debugger<Board, Params, Move> for BarracudaDebug
{
    fn new() -> Self {
        Self {}
    }

    fn sampling_rate() -> Duration {
        Duration::from_secs_f32(0.2)
    }

    fn debug(board: &Board, eval: f32, nodes: u32, depth: u32, pv: &[Move]) {
        print!("pwin: {} visits: {} depth: {} ", eval, nodes, depth);
        print!("pv: ");
        for mv in pv {
            print!("{} ", board.move_to_str(*mv));
        }
        println!();
    }
}
