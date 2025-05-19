use super::{Piece, Square};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ply {
    /// The piece being moved.
    piece: Piece,
    /// The starting square of the piece being moved.
    from: Square,
    /// The ending square of the piece being moved.
    to: Square,
    /// Whether the piece was promoted to a king.
    promoted: bool,
    /// The squares of the pieces that were captured.
    captures: Vec<Square>,
}

impl Ply {
    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn from(&self) -> Square {
        self.from
    }

    pub fn to(&self) -> Square {
        self.to
    }

    pub fn promoted(&self) -> bool {
        self.promoted
    }

    pub fn captures(&self) -> &[Square] {
        &self.captures
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlyBuilder {
    piece: Piece,
    from: Square,
    to: Square,
    promoted: bool,
    captures: Vec<Square>,
}

impl PlyBuilder {
    pub fn new(piece: Piece, from: Square) -> Self {
        Self { piece, from, to: from, promoted: false, captures: Vec::new() }
    }

    pub fn step(mut self, to: Square) -> Self {
        self.to = to;
        self
    }

    pub fn capture(mut self, to: Square, capture: Square) -> Self {
        self.to = to;
        self.captures.push(capture);
        self
    }

    pub fn capture_multiple(mut self, to: Square, captures: Vec<Square>) -> Self {
        self.to = to;
        self.captures.extend(captures);
        self
    }

    pub fn promote(mut self) -> Self {
        self.promoted = true;
        self.piece.promote();
        self
    }

    pub fn build(self) -> Ply {
        Ply { piece: self.piece, from: self.from, to: self.to, promoted: self.promoted, captures: self.captures }
    }
}

impl std::fmt::Display for Ply {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}", self.from, self.to)?;
        if self.promoted {
            write!(f, "p")?;
        }
        for capture in &self.captures {
            write!(f, "x{}", capture)?;
        }
        Ok(())
    }
}
