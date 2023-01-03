use classic_computer_science_problems::board::*;
use classic_computer_science_problems::minimax::*;
use classic_computer_science_problems::tictactoe::*;
use std::io;
use std::io::Write;

fn get_player_move(board: &TTTBoard) -> <TTTBoard as Board>::Move {
    let mut player_move = None;
    while player_move.map_or(true, |x| !board.legal_moves().contains(&x)) {
        print!("Enter a legal square (0-8): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        player_move = input.trim().parse().ok();
    }
    player_move.unwrap()
}

fn main() {
    let mut board = TTTBoard::new();
    loop {
        let human_move = get_player_move(&board);
        board = board.move_(&human_move);
        if board.is_win() {
            println!("You win!");
            break;
        }
        if board.is_draw() {
            println!("Draw!");
            break;
        }
        let computer_move = find_best_move(&board, 9);
        println!("Computer move: {}", computer_move);
        board = board.move_(&computer_move);
        board.display();
        if board.is_win() {
            println!("Computer wins!");
            break;
        }
        if board.is_draw() {
            println!("Draw!");
            break;
        }
    }
}
