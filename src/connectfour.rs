use std::ops::Index;

use crate::board::{Board, Piece};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum C4Piece {
    Red,
    Black,
    Empty,
}

impl Piece for C4Piece {
    fn opposite(&self) -> Self {
        match self {
            C4Piece::Red => C4Piece::Black,
            C4Piece::Black => C4Piece::Red,
            C4Piece::Empty => C4Piece::Empty,
        }
    }
}

fn generate_segments(
    num_columns: usize,
    num_rows: usize,
    segment_length: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut segments = Vec::new();

    // generate the vertical segments
    for c in 0..num_columns {
        for r in 0..num_rows - segment_length + 1 {
            let mut segment = Vec::with_capacity(segment_length);
            for t in 0..segment_length {
                segment.push((c, r + t));
            }
            segments.push(segment);
        }
    }

    // generate the horizontal segments
    for c in 0..num_columns - segment_length + 1 {
        for r in 0..num_rows {
            let mut segment = Vec::with_capacity(segment_length);
            for t in 0..segment_length {
                segment.push((c + t, r));
            }
            segments.push(segment);
        }
    }

    // generate the bottom left to top right diagonal segments
    for c in 0..num_columns - segment_length + 1 {
        for r in 0..num_rows - segment_length + 1 {
            let mut segment = Vec::with_capacity(segment_length);
            for t in 0..segment_length {
                segment.push((c + t, r + t));
            }
            segments.push(segment);
        }
    }

    // generate the top left to bottom right diagonal segments
    for c in 0..num_columns - segment_length + 1 {
        for r in segment_length - 1..num_rows {
            let mut segment = Vec::with_capacity(segment_length);
            for t in 0..segment_length {
                segment.push((c + t, r - t));
            }
            segments.push(segment);
        }
    }

    segments
}

const NUM_ROWS: usize = 6;
const NUM_COLUMNS: usize = 7;
const SEGMENT_LENGTH: usize = 4;

lazy_static! {
    static ref SEGMENTS: Vec<Vec<(usize, usize)>> =
        generate_segments(NUM_COLUMNS, NUM_ROWS, SEGMENT_LENGTH);
}

#[derive(Debug, Clone)]
struct Column {
    container: Vec<C4Piece>,
}

impl Column {
    fn new() -> Self {
        Self {
            container: Vec::new(),
        }
    }

    fn full(&self) -> bool {
        self.container.len() == NUM_ROWS
    }

    fn push(&mut self, item: C4Piece) {
        self.container.push(item);
    }
}

impl Index<usize> for Column {
    type Output = C4Piece;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.container.len() {
            &self.container[index]
        } else {
            &C4Piece::Empty
        }
    }
}

#[derive(Debug, Clone)]
pub struct C4Board {
    position: Vec<Column>,
    turn: C4Piece,
}

impl C4Board {
    pub fn new() -> Self {
        Self {
            position: vec![Column::new(); NUM_COLUMNS],
            turn: C4Piece::Black,
        }
    }

    pub fn display(&self) {
        for i in 0..NUM_COLUMNS {
            print!("{} ", i);
        }
        println!("");
        for r in (0..NUM_ROWS).rev() {
            for c in 0..NUM_COLUMNS {
                print!(
                    "{}",
                    match self.position[c][r] {
                        C4Piece::Red => "R",
                        C4Piece::Black => "B",
                        C4Piece::Empty => " ",
                    }
                );
                if c < NUM_COLUMNS - 1 {
                    print!("|");
                }
            }
            println!();
        }
        println!("-------------");
    }

    fn count_segment(&self, segment: &[(usize, usize)]) -> (usize, usize) {
        let mut red_count = 0;
        let mut black_count = 0;
        for (c, r) in segment.iter() {
            match self.position[*c][*r] {
                C4Piece::Red => red_count += 1,
                C4Piece::Black => black_count += 1,
                C4Piece::Empty => (),
            }
        }
        (black_count, red_count)
    }

    fn evaluate_segment(&self, segment: &[(usize, usize)], player: &<Self as Board>::Piece) -> f64 {
        let (black_count, red_count) = self.count_segment(segment);
        if red_count > 0 && black_count > 0 {
            return 0.0;
        }
        let count = red_count.max(black_count);
        let mut score = 0.0;
        if count == 2 {
            score += 1.0;
        } else if count == 3 {
            score += 100.0;
        } else if count == 4 {
            score += 1_000_000.0;
        }
        let advantaged = if red_count > black_count {
            C4Piece::Red
        } else {
            C4Piece::Black
        };
        if *player != advantaged {
            score *= -1.0;
        }
        score
    }
}

impl Board for C4Board {
    type Piece = C4Piece;
    type Move = usize;

    fn turn(&self) -> &Self::Piece {
        &self.turn
    }

    fn move_(&self, location: &Self::Move) -> Self {
        let mut new_position = self.position.clone();
        new_position[*location].push(self.turn);
        Self {
            position: new_position,
            turn: self.turn.opposite(),
        }
    }

    fn legal_moves(&self) -> Vec<usize> {
        let mut moves = Vec::new();
        for c in 0..NUM_COLUMNS {
            if !self.position[c].full() {
                moves.push(c);
            }
        }
        moves
    }

    fn is_win(&self) -> bool {
        for segment in SEGMENTS.iter() {
            let (black_count, red_count) = self.count_segment(segment);
            if red_count == SEGMENT_LENGTH || black_count == SEGMENT_LENGTH {
                return true;
            }
        }
        false
    }

    fn evaluate(&self, player: &Self::Piece) -> f64 {
        let mut total = 0.0;
        for segment in SEGMENTS.iter() {
            total += self.evaluate_segment(segment, player);
        }
        total
    }
}
