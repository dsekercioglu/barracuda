use crate::barracuda::traits::{BarracudaBoard, BarracudaMove, BarracudaParams, GameState, Player};

#[derive(Default)]
pub struct TicTacToeParams;

impl BarracudaParams for TicTacToeParams {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TicTacToeBoard {
    board: [[Option<Player>; 3]; 3],
    turn: Player,
    cnt: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Square {
    x: usize,
    y: usize,
}

impl BarracudaMove for Square {}

impl BarracudaBoard<TicTacToeParams, Square> for TicTacToeBoard {
    fn new(_: TicTacToeParams) -> Self {
        Self {
            board: [[None; 3]; 3],
            turn: Player::P1,
            cnt: 0,
        }
    }

    fn make_move(&mut self, mv: Square) {
        self.board[mv.x][mv.y] = Some(self.turn);
        self.turn = match self.turn {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        };
        self.cnt += 1;
    }

    fn get_moves(&self) -> Vec<Square> {
        let mut moves = vec![];
        for x in 0..3 {
            for y in 0..3 {
                if self.board[x][y].is_none() {
                    moves.push(Square { x, y })
                }
            }
        }
        moves
    }

    fn game_state(&self) -> GameState {
        for x in 0..3 {
            let root = self.board[x][0];
            if root.is_some() && self.board[x][1] == root && self.board[x][2] == root {
                return GameState::End(root);
            }
        }
        for y in 0..3 {
            let root = self.board[0][y];
            if root.is_some() && self.board[1][y] == root && self.board[2][y] == root {
                return GameState::End(root);
            }
        }
        let root = self.board[0][0];
        if root.is_some() && self.board[1][1] == root && self.board[2][2] == root {
            return GameState::End(root);
        }
        let root = self.board[2][0];
        if root.is_some() && self.board[1][1] == root && self.board[0][2] == root {
            return GameState::End(root);
        }

        if self.cnt == 9 {
            GameState::End(None)
        } else {
            GameState::Ongoing
        }
    }

    fn turn(&self) -> Player {
        self.turn
    }

    fn move_to_str(&self, mv: Square) -> String {
        format!("({}, {})", mv.x, mv.y)
    }
}
