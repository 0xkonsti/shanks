use std::io::Write;

use shanks_core::board::{Board, Color};
use shanks_engine::Engine;

use crate::cli::PlayArgs;

pub fn play(args: PlayArgs) {
    let mut board = Board::default();
    let mut engine = Engine::new(Color::White);
    print!("{esc}c", esc = 27 as char);
    println!("Playing a game of checkers...");
    println!("{}", board);
    println!("Value: {}", engine.evaluate(&board));
    println!("{} to move", board.to_move());
    println!("Legal plies:");
    board.legal_plies();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut input = String::new();

    loop {
        //println!("{}", board.get_gamestate());
        print!("Enter a ply index (or 'exit' to quit): ");
        stdout.flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        }
        let index: usize = match input.trim().parse() {
            Ok(index) => index,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if let Some(ply) = board.get_ply(index) {
            print!("{esc}c", esc = 27 as char);
            println!("Selected ply: {}", ply);
            board.ply(ply.clone());
            let gamestate = board.get_gamestate();
            if gamestate.is_over() {
                println!("Game over! {}", gamestate);
                break;
            }
            println!("{}", board);
            println!("Value: {}", engine.evaluate(&board));
            println!("{} to move", board.to_move());
            println!("Legal plies:");
            board.legal_plies();
        } else {
            println!("No ply found at index {}", index);
        }
    }
}
