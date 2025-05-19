use colored::Colorize;

use super::{
    backend::{bitboard::BitBoard, Backend},
    Color, Ply, Square,
};

pub struct Board {
    backend: Box<dyn Backend>,

    current_color: Color,
    selected: Option<Square>,
    legal_plies: Vec<Ply>,
}

impl Board {
    pub fn new(backend: Box<dyn Backend>) -> Self {
        let legal_plies = backend.get_legal_plies(Color::White);
        Self { backend, current_color: Color::White, selected: None, legal_plies }
    }

    /// Carries out the given ply on the board.
    /// The given ply is assumed to be legal and will not be further validated here.
    pub fn ply(&mut self, ply: Ply) {
        self.backend.ply(ply);

        self.current_color.swap();
        self.selected = None;
        self.legal_plies = self.backend.get_legal_plies(self.current_color);
    }

    pub fn legal_plies(&self) {
        for (i, ply) in self.legal_plies.iter().enumerate() {
            println!("{}: {}", i, ply);
        }
    }

    pub fn get_ply(&self, index: usize) -> Option<&Ply> {
        self.legal_plies.get(index)
    }

    pub fn to_move(&self) -> Color {
        self.current_color
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(Box::new(BitBoard::default()))
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square = Square::ALL[rank + file * 8];
                let mut symbol = match self.backend.get_piece(square) {
                    Some(piece) => format!("{} ", piece),
                    //None => write!(f, ". ")?,
                    None => format!("{} ", '\u{00B7}'), // Using a middle dot for empty squares
                };
                if self.selected == Some(square) {
                    symbol = symbol.bold().green().to_string();
                }
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  a b c d e f g h")?;
        Ok(())
    }
}
