pub trait Piece {
    fn opposite(&self) -> Self;
}

pub trait Board {
    type Piece: Piece;
    type Move: Eq;

    fn turn(&self) -> &Self::Piece;

    fn move_(&self, location: &Self::Move) -> Self;

    fn legal_moves(&self) -> Vec<Self::Move>;

    fn is_win(&self) -> bool;

    fn is_draw(&self) -> bool {
        !self.is_win() && self.legal_moves().is_empty()
    }

    fn evaluate(&self, player: &Self::Piece) -> f64;
}
