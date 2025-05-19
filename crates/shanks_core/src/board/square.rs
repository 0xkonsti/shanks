#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square {
    file: u8,
    rank: u8,
}

#[rustfmt::skip]
impl Square {
    pub const A1: Square = Square { file: 0, rank: 0 };
    pub const A2: Square = Square { file: 0, rank: 1 };
    pub const A3: Square = Square { file: 0, rank: 2 };
    pub const A4: Square = Square { file: 0, rank: 3 };
    pub const A5: Square = Square { file: 0, rank: 4 };
    pub const A6: Square = Square { file: 0, rank: 5 };
    pub const A7: Square = Square { file: 0, rank: 6 };
    pub const A8: Square = Square { file: 0, rank: 7 };
    pub const B1: Square = Square { file: 1, rank: 0 };
    pub const B2: Square = Square { file: 1, rank: 1 };
    pub const B3: Square = Square { file: 1, rank: 2 };
    pub const B4: Square = Square { file: 1, rank: 3 };
    pub const B5: Square = Square { file: 1, rank: 4 };
    pub const B6: Square = Square { file: 1, rank: 5 };
    pub const B7: Square = Square { file: 1, rank: 6 };
    pub const B8: Square = Square { file: 1, rank: 7 };
    pub const C1: Square = Square { file: 2, rank: 0 };
    pub const C2: Square = Square { file: 2, rank: 1 };
    pub const C3: Square = Square { file: 2, rank: 2 };
    pub const C4: Square = Square { file: 2, rank: 3 };
    pub const C5: Square = Square { file: 2, rank: 4 };
    pub const C6: Square = Square { file: 2, rank: 5 };
    pub const C7: Square = Square { file: 2, rank: 6 };
    pub const C8: Square = Square { file: 2, rank: 7 };
    pub const D1: Square = Square { file: 3, rank: 0 };
    pub const D2: Square = Square { file: 3, rank: 1 };
    pub const D3: Square = Square { file: 3, rank: 2 };
    pub const D4: Square = Square { file: 3, rank: 3 };
    pub const D5: Square = Square { file: 3, rank: 4 };
    pub const D6: Square = Square { file: 3, rank: 5 };
    pub const D7: Square = Square { file: 3, rank: 6 };
    pub const D8: Square = Square { file: 3, rank: 7 };
    pub const E1: Square = Square { file: 4, rank: 0 };
    pub const E2: Square = Square { file: 4, rank: 1 };
    pub const E3: Square = Square { file: 4, rank: 2 };
    pub const E4: Square = Square { file: 4, rank: 3 };
    pub const E5: Square = Square { file: 4, rank: 4 };
    pub const E6: Square = Square { file: 4, rank: 5 };
    pub const E7: Square = Square { file: 4, rank: 6 };
    pub const E8: Square = Square { file: 4, rank: 7 };
    pub const F1: Square = Square { file: 5, rank: 0 };
    pub const F2: Square = Square { file: 5, rank: 1 };
    pub const F3: Square = Square { file: 5, rank: 2 };
    pub const F4: Square = Square { file: 5, rank: 3 };
    pub const F5: Square = Square { file: 5, rank: 4 };
    pub const F6: Square = Square { file: 5, rank: 5 };
    pub const F7: Square = Square { file: 5, rank: 6 };
    pub const F8: Square = Square { file: 5, rank: 7 };
    pub const G1: Square = Square { file: 6, rank: 0 };
    pub const G2: Square = Square { file: 6, rank: 1 };
    pub const G3: Square = Square { file: 6, rank: 2 };
    pub const G4: Square = Square { file: 6, rank: 3 };
    pub const G5: Square = Square { file: 6, rank: 4 };
    pub const G6: Square = Square { file: 6, rank: 5 };
    pub const G7: Square = Square { file: 6, rank: 6 };
    pub const G8: Square = Square { file: 6, rank: 7 };
    pub const H1: Square = Square { file: 7, rank: 0 };
    pub const H2: Square = Square { file: 7, rank: 1 };
    pub const H3: Square = Square { file: 7, rank: 2 };
    pub const H4: Square = Square { file: 7, rank: 3 };
    pub const H5: Square = Square { file: 7, rank: 4 };
    pub const H6: Square = Square { file: 7, rank: 5 };
    pub const H7: Square = Square { file: 7, rank: 6 };
    pub const H8: Square = Square { file: 7, rank: 7 };

    pub const ALL: [Square; 64] = [
        Self::A1, Self::A2, Self::A3, Self::A4, Self::A5, Self::A6, Self::A7, Self::A8,
        Self::B1, Self::B2, Self::B3, Self::B4, Self::B5, Self::B6, Self::B7, Self::B8,
        Self::C1, Self::C2, Self::C3, Self::C4, Self::C5, Self::C6, Self::C7, Self::C8,
        Self::D1, Self::D2, Self::D3, Self::D4, Self::D5, Self::D6, Self::D7, Self::D8,
        Self::E1, Self::E2, Self::E3, Self::E4, Self::E5, Self::E6, Self::E7, Self::E8,
        Self::F1, Self::F2, Self::F3, Self::F4, Self::F5, Self::F6, Self::F7, Self::F8,
        Self::G1, Self::G2, Self::G3, Self::G4, Self::G5, Self::G6, Self::G7, Self::G8,
        Self::H1, Self::H2, Self::H3, Self::H4, Self::H5, Self::H6, Self::H7, Self::H8,
    ];

    pub fn file(&self) -> u8 {
        self.file
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }

    pub fn index(&self) -> usize {
        (self.rank * 8 + self.file) as usize
    }

    pub fn moved_by(&self, file: i8, rank: i8) -> Option<Square> {
        let new_file = self.file as i8 + file;
        let new_rank = self.rank as i8 + rank;
        if new_file < 0 || new_file > 7 || new_rank < 0 || new_rank > 7 {
            return None;
        }
        Some(Square { file: new_file as u8, rank: new_rank as u8 })
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let file = (self.file + 'a' as u8) as char;
        let rank = self.rank + 1;
        write!(f, "{}{}", file, rank)
    }
}
