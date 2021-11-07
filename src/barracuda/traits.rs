use std::hash::Hash;

pub trait BarracudaAlgorithm: Send {}

pub trait BarracudaMove: Copy + Hash + PartialEq + Eq + Send + 'static {}

pub trait BarracudaParams: Send + Default + 'static {}

pub trait BarracudaBoard<Params, Move>: Clone + Hash + PartialEq + Eq + Send + 'static {
    fn new(params: Params) -> Self;

    fn make_move(&mut self, mv: Move);

    fn get_moves(&self) -> Vec<Move>;

    fn game_state(&self) -> GameState;

    fn turn(&self) -> Player;

    fn move_to_str(&self, mv: Move) -> String;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Player {
    P1,
    P2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Ongoing,
    End(Option<Player>),
}
