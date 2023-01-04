use crate::board::Board;

pub fn minimax<B: Board>(
    board: &B,
    maximizing: bool,
    original_player: &B::Piece,
    max_depth: usize,
) -> f64 {
    if board.is_win() || board.is_draw() || max_depth == 0 {
        return board.evaluate(original_player);
    }

    if maximizing {
        let mut best_eval = f64::NEG_INFINITY;
        for move_ in board.legal_moves() {
            let result = minimax(&board.move_(&move_), false, original_player, max_depth - 1);
            best_eval = result.max(best_eval)
        }
        best_eval
    } else {
        let mut worst_eval = f64::INFINITY;
        for move_ in board.legal_moves() {
            let result = minimax(&board.move_(&move_), true, original_player, max_depth - 1);
            worst_eval = result.min(worst_eval);
        }
        worst_eval
    }
}

pub fn alphabeta<B: Board>(
    board: &B,
    maximizing: bool,
    original_player: &B::Piece,
    max_depth: usize,
    mut alpha: f64,
    mut beta: f64,
) -> f64 {
    if board.is_win() || board.is_draw() || max_depth == 0 {
        return board.evaluate(original_player);
    }

    if maximizing {
        for move_ in board.legal_moves() {
            let result = alphabeta(
                &board.move_(&move_),
                false,
                original_player,
                max_depth - 1,
                alpha,
                beta,
            );
            alpha = result.max(alpha);
            if beta <= alpha {
                break;
            }
        }
        alpha
    } else {
        for move_ in board.legal_moves() {
            let result = alphabeta(
                &board.move_(&move_),
                true,
                original_player,
                max_depth - 1,
                alpha,
                beta,
            );
            beta = result.min(beta);
            if beta <= alpha {
                break;
            }
        }
        beta
    }
}

pub fn find_best_move<B: Board>(board: &B, max_depth: usize) -> B::Move {
    let mut best_eval = f64::NEG_INFINITY;
    let mut best_move = None;
    for move_ in board.legal_moves() {
        let result = alphabeta(
            &board.move_(&move_),
            false,
            board.turn(),
            max_depth - 1,
            f64::NEG_INFINITY,
            f64::INFINITY,
        );
        if result > best_eval {
            best_eval = result;
            best_move = Some(move_);
        }
    }
    best_move.unwrap()
}
