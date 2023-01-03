use crate::board::{Board, Piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TTTPiece {
    X,
    O,
    Empty,
}

impl Piece for TTTPiece {
    fn opposite(&self) -> Self {
        match self {
            TTTPiece::X => TTTPiece::O,
            TTTPiece::O => TTTPiece::X,
            TTTPiece::Empty => TTTPiece::Empty,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TTTBoard {
    position: [TTTPiece; 9],
    turn: TTTPiece,
}

impl TTTBoard {
    pub fn new() -> Self {
        Self {
            position: [TTTPiece::Empty; 9],
            turn: TTTPiece::X,
        }
    }

    pub fn display(&self) {
        for i in 0..9 {
            print!(
                "{}",
                match self.position[i] {
                    TTTPiece::X => "X",
                    TTTPiece::O => "O",
                    TTTPiece::Empty => " ",
                }
            );
            if i % 3 == 2 {
                println!();
            } else {
                print!("|");
            }
        }
    }
}

impl Board for TTTBoard {
    type Piece = TTTPiece;
    type Move = usize;

    fn turn(&self) -> &Self::Piece {
        &self.turn
    }

    fn move_(&self, location: &Self::Move) -> Self {
        let mut temp_position = self.position.clone();
        temp_position[*location] = self.turn;
        Self {
            position: temp_position,
            turn: self.turn.opposite(),
        }
    }

    fn legal_moves(&self) -> Vec<Self::Move> {
        self.position
            .iter()
            .enumerate()
            .filter_map(|(i, p)| if *p == TTTPiece::Empty { Some(i) } else { None })
            .collect()
    }

    fn is_win(&self) -> bool {
        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        lines.iter().any(|line| {
            self.position[line[0]] != TTTPiece::Empty
                && self.position[line[0]] == self.position[line[1]]
                && self.position[line[1]] == self.position[line[2]]
        })
    }

    fn evaluate(&self, player: &Self::Piece) -> f64 {
        if self.is_win() {
            if self.turn == *player {
                -1.0
            } else {
                1.0
            }
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::minimax::find_best_move;

    use super::*;

    #[test]
    fn test_easy_position() {
        let to_win_easy_position = [
            TTTPiece::X,
            TTTPiece::O,
            TTTPiece::X,
            TTTPiece::X,
            TTTPiece::Empty,
            TTTPiece::O,
            TTTPiece::Empty,
            TTTPiece::Empty,
            TTTPiece::O,
        ];
        let test_board = TTTBoard {
            position: to_win_easy_position,
            turn: TTTPiece::X,
        };
        let answer = find_best_move(&test_board, 100);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_block_position() {
        let to_block_position = [
            TTTPiece::X,
            TTTPiece::Empty,
            TTTPiece::Empty,
            TTTPiece::Empty,
            TTTPiece::Empty,
            TTTPiece::O,
            TTTPiece::Empty,
            TTTPiece::X,
            TTTPiece::O,
        ];
        let test_board = TTTBoard {
            position: to_block_position,
            turn: TTTPiece::X,
        };
        let answer = find_best_move(&test_board, 100);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_hard_position() {
        let to_win_hard_position = [
            TTTPiece::X,
            TTTPiece::Empty,
            TTTPiece::Empty,
            TTTPiece::Empty,
            TTTPiece::Empty,
            TTTPiece::O,
            TTTPiece::O,
            TTTPiece::X,
            TTTPiece::Empty,
        ];
        let test_board = TTTBoard {
            position: to_win_hard_position,
            turn: TTTPiece::X,
        };
        let answer = find_best_move(&test_board, 100);
        assert_eq!(answer, 1);
    }
}
