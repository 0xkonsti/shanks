const WHITE_MOVES: [(i8, i8); 2] = [(-1, 1), (1, 1)];
const BLACK_MOVES: [(i8, i8); 2] = [(-1, -1), (1, -1)];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Color {
    #[default]
    White = 0,
    Black = 1,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn swap(&mut self) -> Self {
        *self = self.opposite();
        *self
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Color::White => "White",
            Color::Black => "Black",
        };
        write!(f, "{}", s)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum PieceKind {
    #[default]
    Man = 0,
    King = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
}

impl Piece {
    pub const BLACK_KING: Self = Self { color: Color::Black, kind: PieceKind::King };
    pub const BLACK_MAN: Self = Self { color: Color::Black, kind: PieceKind::Man };
    pub const WHITE_KING: Self = Self { color: Color::White, kind: PieceKind::King };
    pub const WHITE_MAN: Self = Self { color: Color::White, kind: PieceKind::Man };

    pub fn new(color: Color, kind: PieceKind) -> Self {
        Self { color, kind }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn kind(&self) -> PieceKind {
        self.kind
    }

    pub fn promote(&mut self) {
        self.kind = PieceKind::King;
    }

    pub fn is_king(&self) -> bool {
        self.kind == PieceKind::King
    }

    pub fn moves(&self) -> Vec<(i8, i8)> {
        let mut moves = Vec::new();
        match self.color {
            Color::White => {
                moves.extend_from_slice(&WHITE_MOVES);
                if self.is_king() {
                    moves.extend_from_slice(&BLACK_MOVES);
                }
            }
            Color::Black => {
                moves.extend_from_slice(&BLACK_MOVES);
                if self.is_king() {
                    moves.extend_from_slice(&WHITE_MOVES);
                }
            }
        }
        moves
    }

    pub fn promotion_rank(&self) -> u8 {
        match self.color {
            Color::White => 7,
            Color::Black => 0,
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "{}", char::from(*self))
        let c = match self.kind {
            PieceKind::Man => {
                if self.color == Color::White {
                    '\u{026C0}'
                } else {
                    '\u{026C2}'
                }
            }
            PieceKind::King => {
                if self.color == Color::White {
                    '\u{026C1}'
                } else {
                    '\u{026C3}'
                }
            }
        };
        write!(f, "{}", c)
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> Self {
        let mut c = match piece.kind {
            PieceKind::Man => 'm',
            PieceKind::King => 'k',
        };
        if piece.color == Color::White {
            c = c.to_ascii_uppercase();
        }
        c
    }
}
