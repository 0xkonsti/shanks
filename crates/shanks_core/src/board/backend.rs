pub(crate) mod bitboard;

use super::{Color, Piece, Ply, Square};

pub trait Backend {
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
}
