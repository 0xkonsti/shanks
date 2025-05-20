pub(crate) mod bitboard;

use std::hash::{Hash, Hasher};

use super::{Color, GameState, Piece, Ply, Square};

pub trait Backend: BackendClone {
    /// Returns the current game state.
    fn get_gamestate(&self) -> GameState;

    /// Returns the piece at the given square.
    /// If there is no piece at the square, returns None.
    fn get_piece(&self, square: Square) -> Option<Piece>;

    /// Sets the piece at the given square.
    /// If there is already a piece at the square, it is replaced.
    fn set_piece(&mut self, square: Square, piece: Piece);

    /// Removes the piece at the given square and returns it.
    /// If there is no piece at the square, returns None.
    fn remove_piece(&mut self, square: Square) -> Option<Piece>;

    /// Removes the pieces at the given squares and returns them.
    ///
    /// Can be overridden to optimize for performance if needed.
    fn remove_pieces(&mut self, squares: &[Square]) -> Vec<Piece> {
        squares.iter().filter_map(|&square| self.remove_piece(square)).collect()
    }

    /// Returns a vector of all legal Plys for the player with the given color.
    /// A ply must be complete, meaning it must include all captures and promotions,
    /// after which a player's turn is over (i.e. contains all conseqetive captures).
    fn get_legal_plies(&self, color: Color) -> Vec<Ply>;

    /// Carries out the given ply on the backend board.
    /// The given ply is assumed to be legal and will not be further validated here.
    fn ply(&mut self, ply: Ply) {
        self.remove_piece(ply.from());
        self.set_piece(ply.to(), ply.piece());
        self.remove_pieces(ply.captures());
    }

    /// Returns the number of man pieces for the given color.
    fn man_count(&self, color: Color) -> u8;

    /// Returns the number of king pieces for the given color.
    fn king_count(&self, color: Color) -> u8;

    /// Returns a hashed representation of the current state of the board.
    fn state_hash(&self) -> u64;
}

pub trait BackendClone {
    fn clone_box(&self) -> Box<dyn Backend>;
}

impl<T> BackendClone for T
where
    T: 'static + Backend + Clone,
{
    fn clone_box(&self) -> Box<dyn Backend> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Backend> {
    fn clone(&self) -> Box<dyn Backend> {
        self.clone_box()
    }
}

impl Hash for Box<dyn Backend> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state_hash().hash(state);
    }
}
