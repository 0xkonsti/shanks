use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
};

use shanks_util::util::*;

use super::Backend;
use crate::board::{Color, GameState, Piece, PieceKind, Ply, PlyBuilder, Square};

const DEFAULT_BOARD_WHITE: u64 = 0x000000000055aa55;
const DEFAULT_BOARD_BLACK: u64 = 0xaa55aa0000000000;
const DEFAULT_BOARD_KINGS: u64 = 0x0000000000000000;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PliesState {
    state: u64,
    color: Color,
    plies: Vec<Ply>,
}

impl PliesState {
    fn new() -> Self {
        Self { state: 0, color: Color::White, plies: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitBoard {
    white: BitField,
    black: BitField,
    kings: BitField,

    legal_plies: RefCell<PliesState>,
}

impl BitBoard {
    fn get_color_field(&self, color: Color) -> &BitField {
        match color {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }

    fn get_color_field_mut(&mut self, color: Color) -> &mut BitField {
        match color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        }
    }

    fn get_legal_plies_for(&self, square: Square, color: Color, only_captures: bool) -> (bool, Vec<Ply>) {
        let Some(piece) = self.get_piece(square) else {
            return (false, Vec::new());
        };
        if piece.color() != color {
            return (false, Vec::new());
        }

        let all = self.white.union(self.black);
        let allies = self.get_color_field(piece.color());

        let mut plies = Vec::new();
        let mut captures = only_captures;

        for (dx, dy) in piece.moves() {
            let Some(target) = square.moved_by(dx, dy) else {
                continue;
            };
            let index = target.index();

            let mut pb = PlyBuilder::new(piece, square);

            if all.get(index) {
                if allies.get(index) {
                    continue;
                }
                let Some(jump_target) = target.moved_by(dx, dy) else {
                    continue;
                };
                if all.get(jump_target.index()) {
                    continue;
                }

                if !captures {
                    // If we added any plies to the list before, it had to be a non-capturing moves
                    // which needs to be cleared
                    plies.clear();
                }
                captures = true;

                pb = pb.capture(jump_target, target);
                if piece.promotion_rank() == jump_target.rank() {
                    pb = pb.promote();
                }

                let mut new_board = self.clone();
                new_board.ply(pb.clone().build());

                let (_, square_plies) = new_board.get_legal_plies_for(jump_target, color, true);
                if !square_plies.is_empty() {
                    for square_ply in square_plies {
                        let mut new_pb = pb.clone();
                        new_pb = new_pb.capture_multiple(square_ply.to(), square_ply.captures().to_vec());
                        if square_ply.promoted() {
                            new_pb = new_pb.promote();
                        }
                        plies.push(new_pb.build());
                    }
                    continue;
                }

                plies.push(pb.build());
            } else if !captures {
                // If the target square is empty and we are not capturing, we can move there
                pb = pb.step(target);
                if piece.promotion_rank() == target.rank() {
                    pb = pb.promote();
                }
                plies.push(pb.build());
            }
        }

        (captures, plies)
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        Self {
            white: BitField::new(DEFAULT_BOARD_WHITE),
            black: BitField::new(DEFAULT_BOARD_BLACK),
            kings: BitField::new(DEFAULT_BOARD_KINGS),

            legal_plies: RefCell::new(PliesState::new()),
        }
    }
}

impl Hash for BitBoard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.white.hash(state);
        self.black.hash(state);
        self.kings.hash(state);
    }
}

impl Backend for BitBoard {
    fn get_gamestate(&self) -> GameState {
        let legal_plies = self.legal_plies.borrow();
        if self.white.count() == 0 {
            GameState::Win(Color::Black)
        } else if self.black.count() == 0 {
            GameState::Win(Color::White)
        } else if legal_plies.color == Color::White && legal_plies.plies.is_empty() {
            GameState::Win(Color::Black)
        } else if legal_plies.color == Color::Black && legal_plies.plies.is_empty() {
            GameState::Win(Color::White)
        } else {
            GameState::OnGoing
        }
        // TODO: Implement draw conditions
    }

    fn get_piece(&self, square: Square) -> Option<Piece> {
        let index = square.index();

        let kind = if self.kings.get(index) { PieceKind::King } else { PieceKind::Man };
        if self.white.get(index) {
            Some(Piece::new(Color::White, kind))
        } else if self.black.get(index) {
            Some(Piece::new(Color::Black, kind))
        } else {
            None
        }
    }

    fn set_piece(&mut self, square: Square, piece: Piece) {
        let index = square.index();

        let color_field = self.get_color_field_mut(piece.color());

        color_field.set(index);

        if piece.kind() == PieceKind::King {
            self.kings.set(index);
        } else {
            self.kings.unset(index);
        }
    }

    fn remove_piece(&mut self, square: Square) -> Option<Piece> {
        let index = square.index();
        let piece = self.get_piece(square);
        if piece.is_some() {
            self.white.unset(index);
            self.black.unset(index);
            self.kings.unset(index);
        }
        piece
    }

    fn remove_pieces(&mut self, squares: &[Square]) -> Vec<Piece> {
        let mut removed_pieces = Vec::new();
        let mut remove_mask = BitField::EMPTY;
        for square in squares {
            if let Some(piece) = self.get_piece(*square) {
                removed_pieces.push(piece);
                remove_mask.set(square.index());
            }
        }
        self.white = self.white.difference(remove_mask);
        self.black = self.black.difference(remove_mask);
        self.kings = self.kings.difference(remove_mask);
        removed_pieces
    }

    fn get_legal_plies(&self, color: Color) -> Vec<Ply> {
        // Check cached plies
        {
            let legal_plies = self.legal_plies.borrow();
            if legal_plies.state == self.get_hash() && legal_plies.color == color {
                println!("Using cached plies");
                return legal_plies.plies.clone();
            }
        }
        self.legal_plies.replace(PliesState::new());
        let mut plies = Vec::new();
        let mut capturing = false;

        for square in Square::ALL {
            let (captured, square_plies) = self.get_legal_plies_for(square, color, capturing);
            if captured && !capturing {
                // If we found a capture, we need to clear all non-capturing moves
                plies.clear();
                capturing = true;
            }
            plies.extend(square_plies);
        }

        // Update cache
        {
            let mut legal_plies = self.legal_plies.borrow_mut();
            legal_plies.state = self.get_hash();
            legal_plies.color = color;
            legal_plies.plies = plies.clone();
        }

        plies
    }

    fn man_count(&self, color: Color) -> u8 {
        self.get_color_field(color).count() as u8
    }

    fn king_count(&self, color: Color) -> u8 {
        self.get_color_field(color).intersection(self.kings).count() as u8
    }
}
