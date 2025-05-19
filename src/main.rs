//use std::io::Write;
//
//use shanks_core::board::Board;
//
//fn main() {
//    let mut board = Board::default();
//
//    println!("Hello, world!");
//    println!("{}", board);
//    board.legal_plies();
//    //let plies = board.backend.get_legal_plies(shanks_core::board::Color::White);
//
//    let stdin = std::io::stdin();
//    let mut stdout = std::io::stdout();
//    let mut input = String::new();
//
//    loop {
//        print!("Enter a ply index (or 'exit' to quit): ");
//        stdout.flush().unwrap();
//        input.clear();
//        stdin.read_line(&mut input).unwrap();
//        if input.trim() == "exit" {
//            break;
//        }
//        let index: usize = match input.trim().parse() {
//            Ok(index) => index,
//            Err(_) => {
//                println!("Invalid input. Please enter a number.");
//                continue;
//            }
//        };
//
//        if let Some(ply) = board.get_ply(index) {
//            println!("Selected ply: {}", ply);
//            board.ply(ply.clone());
//            println!("{}", board);
//            board.legal_plies();
//        } else {
//            println!("No ply found at index {}", index);
//        }
//    }
//}

use clap::Parser;
use shanks_cli::{Cli, SubCommand};

fn main() {
    let cli = Cli::parse();

    match cli.subcmd {
        SubCommand::Debug(args) => shanks_cli::debug(args),
        SubCommand::Play(args) => shanks_cli::play(args),
        _ => println!("Not implemented yet"),
    }
}
