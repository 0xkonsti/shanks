/// A simple bitfield wrapper around a `u64`, supporting common bitwise operations.
///
/// `BitField` allows individual bit manipulation and set operations, and is useful
/// for compactly storing multiple boolean flags.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BitField(u64);

impl BitField {
    /// A constant representing an empty `BitField` (no bits set).
    pub const EMPTY: Self = BitField(0);
    /// A constant array of `BitField` values representing each file in an 8x8 chessboard.
    pub const FILES: [BitField; 8] = [
        BitField(0x0101010101010101),
        BitField(0x0202020202020202),
        BitField(0x0404040404040404),
        BitField(0x0808080808080808),
        BitField(0x1010101010101010),
        BitField(0x2020202020202020),
        BitField(0x4040404040404040),
        BitField(0x8080808080808080),
    ];
    /// A constant representing a full `BitField` (all bits set).
    pub const FULL: Self = BitField(u64::MAX);
    /// A constant array of `BitField` values representing each rank in an 8x8 chessboard.
    pub const RANKS: [BitField; 8] = [
        BitField(0x00000000000000FF),
        BitField(0x000000000000FF00),
        BitField(0x0000000000FF0000),
        BitField(0x00000000FF000000),
        BitField(0x000000FF00000000),
        BitField(0x0000FF0000000000),
        BitField(0x00FF000000000000),
        BitField(0xFF00000000000000),
    ];

    /// Creates a new `BitField` from the given `u64` value.
    ///
    /// # Example
    /// ```
    /// let bf = BitField::new(0b101);
    /// ```
    pub fn new(value: u64) -> Self {
        BitField(value)
    }

    /// Returns `true` if the bit at `index` is set.
    ///
    /// # Panics
    /// May panic if `index >= 64`.
    #[inline]
    pub fn get(self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
    }

    /// Sets the bit at `index` to `1`.
    ///
    /// # Panics
    /// May panic if `index >= 64`.
    #[inline]
    pub fn set(&mut self, index: usize) {
        self.0 |= 1 << index;
    }

    /// Sets the bit at `index` to `0`.
    ///
    /// # Panics
    /// May panic if `index >= 64`.
    #[inline]
    pub fn unset(&mut self, index: usize) {
        self.0 &= !(1 << index);
    }

    /// Toggles the bit at `index` (sets it to `1` if it was `0`, and vice versa).
    ///
    /// # Panics
    /// May panic if `index >= 64`.
    #[inline]
    pub fn toggle(&mut self, index: usize) {
        self.0 ^= 1 << index;
    }

    /// Returns `true` if no bits are set in the `BitField`.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns the number of bits set to `1` in the `BitField`.
    #[inline]
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the index of the first (least significant) set bit, or `None` if empty.
    #[inline]
    pub fn first(self) -> Option<u32> {
        if self.is_empty() {
            return None;
        }
        Some(self.0.trailing_zeros())
    }

    /// Returns the index of the last (most significant) set bit, or `None` if empty.
    #[inline]
    pub fn last(self) -> Option<u32> {
        if self.is_empty() {
            return None;
        }
        Some(63 - self.0.leading_zeros())
    }

    /// Returns a new `BitField` with only the bits that are set in both fields.
    #[inline]
    pub fn intersection(self, other: Self) -> Self {
        BitField(self.0 & other.0)
    }

    /// Returns a new `BitField` with all bits that are set in either field.
    #[inline]
    pub fn union(self, other: Self) -> Self {
        BitField(self.0 | other.0)
    }

    /// Returns a new `BitField` with bits that are set in `self` but not in `other`.
    #[inline]
    pub fn difference(self, other: Self) -> Self {
        BitField(self.0 & !other.0)
    }

    /// Returns a new `BitField` with bits that are set in one of the fields but not both.
    #[inline]
    pub fn symmetric_difference(self, other: Self) -> Self {
        BitField(self.0 ^ other.0)
    }

    /// Returns `true` if all bits in `self` are also set in `other`.
    #[inline]
    pub fn is_subset(self, other: Self) -> bool {
        (self.0 & other.0) == self.0
    }

    /// Returns `true` if all bits in `other` are also set in `self`.
    #[inline]
    pub fn is_superset(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns a `Vec` of all indices of bits that are currently set.
    ///
    /// The indices are in ascending order.
    ///
    /// # Example
    /// ```
    /// let bf = BitField::new(0b1010);
    /// assert_eq!(bf.positions(), vec![1, 3]);
    /// ```
    pub fn positions(self) -> Vec<u32> {
        let mut positions = Vec::new();
        let mut value = self.0;
        while value != 0 {
            let index = value.trailing_zeros();
            positions.push(index);
            value &= !(1 << index);
        }
        positions
    }
}

impl std::fmt::Binary for BitField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for BitField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("BitField").field(&format_args!("{:064b}", self)).finish()
    }
}

impl std::fmt::Display for BitField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:064b}", self)
    }
}

impl From<u64> for BitField {
    fn from(value: u64) -> BitField {
        BitField(value)
    }
}

impl From<BitField> for u64 {
    fn from(bitfield: BitField) -> u64 {
        bitfield.0
    }
}

impl std::ops::Deref for BitField {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl std::ops::DerefMut for BitField {
    fn deref_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl std::ops::Add for BitField {
    type Output = BitField;

    fn add(self, rhs: BitField) -> BitField {
        BitField(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for BitField {
    fn add_assign(&mut self, rhs: BitField) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for BitField {
    type Output = BitField;

    fn sub(self, rhs: BitField) -> BitField {
        BitField(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for BitField {
    fn sub_assign(&mut self, rhs: BitField) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul for BitField {
    type Output = BitField;

    fn mul(self, rhs: BitField) -> BitField {
        BitField(self.0 * rhs.0)
    }
}

impl std::ops::MulAssign for BitField {
    fn mul_assign(&mut self, rhs: BitField) {
        self.0 *= rhs.0;
    }
}

impl std::ops::Div for BitField {
    type Output = BitField;

    fn div(self, rhs: BitField) -> BitField {
        BitField(self.0 / rhs.0)
    }
}

impl std::ops::DivAssign for BitField {
    fn div_assign(&mut self, rhs: BitField) {
        self.0 /= rhs.0;
    }
}

impl std::ops::BitAnd for BitField {
    type Output = BitField;

    fn bitand(self, rhs: BitField) -> BitField {
        BitField(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for BitField {
    fn bitand_assign(&mut self, rhs: BitField) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitOr for BitField {
    type Output = BitField;

    fn bitor(self, rhs: BitField) -> BitField {
        BitField(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for BitField {
    fn bitor_assign(&mut self, rhs: BitField) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitXor for BitField {
    type Output = BitField;

    fn bitxor(self, rhs: BitField) -> BitField {
        BitField(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for BitField {
    fn bitxor_assign(&mut self, rhs: BitField) {
        self.0 ^= rhs.0;
    }
}

impl std::ops::Not for BitField {
    type Output = BitField;

    fn not(self) -> BitField {
        BitField(!self.0)
    }
}

impl std::ops::Shl<usize> for BitField {
    type Output = BitField;

    fn shl(self, rhs: usize) -> BitField {
        BitField(self.0 << rhs)
    }
}

impl std::ops::ShlAssign<usize> for BitField {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl std::ops::Shr<usize> for BitField {
    type Output = BitField;

    fn shr(self, rhs: usize) -> BitField {
        BitField(self.0 >> rhs)
    }
}

impl std::ops::ShrAssign<usize> for BitField {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}
