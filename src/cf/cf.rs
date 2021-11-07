use crate::barracuda::traits::{BarracudaBoard, BarracudaMove, BarracudaParams, GameState, Player};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectFour {
    board: Vec<Vec<Option<Player>>>,
    turn: Player,
    width: usize,
    height: usize,
    piece_cnt: usize,
}

impl BarracudaMove for usize {}

const CONSECUTIVE: usize = 4;

pub struct CfParams {
    width: usize,
    height: usize,
}

impl BarracudaParams for CfParams {}

impl Default for CfParams {
    fn default() -> Self {
        Self {
            width: 7,
            height: 6,
        }
    }
}

impl CfParams {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl ConnectFour {
    pub fn board(&self) -> &[Vec<Option<Player>>] {
        &self.board
    }

    fn winner(&self) -> Option<Player> {
        for column in self.board.iter() {
            let mut last = None;
            let mut consecutive = 1usize;
            for &square in column {
                if square == last {
                    consecutive += 1;
                    if consecutive >= CONSECUTIVE {
                        if let Some(player) = last {
                            return Some(player);
                        }
                    }
                } else {
                    consecutive = 1;
                }
                last = square;
            }
        }
        let flat = self.board.iter().flatten();
        for start in 0..self.width {
            let mut last = None;
            let mut consecutive = 1usize;
            for &square in flat.clone().skip(start).step_by(self.height) {
                if square == last {
                    consecutive += 1;
                    if consecutive >= CONSECUTIVE {
                        if let Some(player) = last {
                            return Some(player);
                        }
                    }
                } else {
                    consecutive = 1;
                }
                last = square;
            }
        }
        for x in 0..self.width - CONSECUTIVE + 1 {
            'outer_0: for y in 0..self.height - CONSECUTIVE + 1 {
                if let Some(start) = self.board[x][y] {
                    for j in 1..CONSECUTIVE {
                        if self.board[x + j][y + j] != Some(start) {
                            continue 'outer_0;
                        }
                    }
                    return Some(start);
                }
            }
            'outer_1: for y in CONSECUTIVE - 1..self.height {
                if let Some(start) = self.board[x][y] {
                    for j in 0..CONSECUTIVE {
                        if self.board[x + j][y - j] != Some(start) {
                            continue 'outer_1;
                        }
                    }
                    return Some(start);
                }
            }
        }
        None
    }
}

impl BarracudaBoard<CfParams, usize> for ConnectFour {
    fn new(params: CfParams) -> Self {
        Self {
            width: params.width,
            height: params.height,
            turn: Player::P1,
            board: vec![vec![None; params.height]; params.width],
            piece_cnt: 0,
        }
    }

    fn make_move(&mut self, column: usize) {
        let column = &mut self.board[column];
        let turn = self.turn;
        if let Some(index) = column.iter().position(|&p| p == None) {
            column[index] = Some(turn);
            self.piece_cnt += 1;
        } else {
            panic!();
        }

        self.turn = match self.turn {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        };
    }

    fn get_moves(&self) -> Vec<usize> {
        let mut moves = vec![];
        for (index, col) in self.board.iter().enumerate() {
            if col[col.len() - 1].is_none() {
                moves.push(index);
            }
        }
        moves
    }

    fn game_state(&self) -> GameState {
        let winner = self.winner();
        if self.piece_cnt >= self.width * self.height {
            GameState::End(winner)
        } else if let Some(winner) = winner {
            GameState::End(Some(winner))
        } else {
            GameState::Ongoing
        }
    }

    fn turn(&self) -> Player {
        self.turn
    }

    fn move_to_str(&self, mv: usize) -> String {
        format!("{}", mv)
    }
}
