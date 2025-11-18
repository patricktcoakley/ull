//! Type-safe 16-bit value for addresses with wrapping arithmetic.
//!
//! # Examples
//!
//! ```
//! use ull::Word;
//!
//! let addr = Word(0x8000);
//! assert_eq!(addr + 2, Word(0x8002));
//!
//! // Extract low/high bytes (6502 is little-endian)
//! assert_eq!(addr.lo().0, 0x00);
//! assert_eq!(addr.hi().0, 0x80);
//! ```

use crate::byte::Byte;
use core::fmt::{LowerHex, UpperHex};
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// Type-safe 16-bit value for addresses and multi-byte operands.
///
/// Wraps a `u16` and provides operator overloads with wrapping arithmetic.
/// Primarily used for memory addresses (0x0000-0xFFFF). Use the [`word!`](crate::word!) macro
/// for convenient construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word(pub u16);

/// Convenience macro for creating [`Word`] values.
///
/// # Examples
///
/// ```
/// use ull::word;
///
/// let addr = word!(0x8000);
/// assert_eq!(addr, ull::Word(0x8000));
///
/// // Also works with byte tuples (little-endian)
/// let addr = word!((0x00u8, 0x80u8));
/// assert_eq!(addr, ull::Word(0x8000));
/// ```
#[macro_export]
macro_rules! word {
    ($val:expr) => {
        $crate::word::Word::from($val)
    };
}

impl Word {
    pub const ZERO: Word = Word(0);
    pub const MAX: Word = Word(u16::MAX);

    /// Returns the low byte (bits 0-7).
    #[inline]
    #[must_use]
    pub fn lo(self) -> Byte {
        Byte::from((self.0 & 0x00FF) as u8)
    }

    /// Returns the high byte (bits 8-15).
    #[inline]
    #[must_use]
    pub fn hi(self) -> Byte {
        Byte::from((self.0 >> 8) as u8)
    }

    /// Returns both bytes as (low, high) tuple.
    ///
    /// Useful for little-endian byte operations.
    #[inline]
    #[must_use]
    pub fn lo_hi(self) -> (Byte, Byte) {
        (self.lo(), self.hi())
    }

    /// Converts to `usize` for array indexing.
    #[inline]
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<(u8, u8)> for Word {
    fn from((lo, hi): (u8, u8)) -> Self {
        Word((u16::from(hi) << 8) | u16::from(lo))
    }
}

impl From<(Byte, Byte)> for Word {
    fn from((lo, hi): (Byte, Byte)) -> Self {
        Word(((u16::from(hi)) << 8) | u16::from(lo))
    }
}

impl From<Word> for (Byte, Byte) {
    fn from(value: Word) -> Self {
        value.lo_hi()
    }
}

impl From<Word> for (u8, u8) {
    fn from(value: Word) -> Self {
        let (lo, hi) = value.lo_hi();
        (u8::from(lo), u8::from(hi))
    }
}

impl From<u16> for Word {
    fn from(value: u16) -> Self {
        Word(value)
    }
}

impl From<i16> for Word {
    fn from(value: i16) -> Self {
        Word(value as u16)
    }
}

impl From<u32> for Word {
    fn from(value: u32) -> Self {
        Word((value & 0xFFFF) as u16)
    }
}

impl From<i32> for Word {
    fn from(value: i32) -> Self {
        Word((value as u32 & 0xFFFF) as u16)
    }
}

impl From<Word> for u16 {
    fn from(value: Word) -> Self {
        value.0
    }
}

impl From<Word> for i16 {
    fn from(value: Word) -> Self {
        value.0 as i16
    }
}

impl From<Word> for u32 {
    fn from(value: Word) -> Self {
        u32::from(value.0)
    }
}

impl From<Word> for i32 {
    fn from(value: Word) -> Self {
        i32::from(value.0)
    }
}

impl From<Word> for usize {
    fn from(value: Word) -> Self {
        value.0 as usize
    }
}

impl From<u8> for Word {
    fn from(value: u8) -> Self {
        Word(u16::from(value))
    }
}

impl From<Byte> for Word {
    fn from(value: Byte) -> Self {
        Word(u16::from(value))
    }
}

impl LowerHex for Word {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for Word {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl Add<u16> for Word {
    type Output = Word;

    fn add(self, rhs: u16) -> Word {
        Word(self.0.wrapping_add(rhs))
    }
}

impl AddAssign<u16> for Word {
    fn add_assign(&mut self, rhs: u16) {
        self.0 = self.0.wrapping_add(rhs);
    }
}

impl Add<Byte> for Word {
    type Output = Word;

    fn add(self, rhs: Byte) -> Word {
        Word(self.0.wrapping_add(u16::from(rhs)))
    }
}

impl AddAssign<Byte> for Word {
    fn add_assign(&mut self, rhs: Byte) {
        self.0 = self.0.wrapping_add(u16::from(rhs));
    }
}

impl Add for Word {
    type Output = Word;

    fn add(self, rhs: Word) -> Word {
        Word(self.0.wrapping_add(rhs.0))
    }
}

impl Add<i8> for Word {
    type Output = Word;

    fn add(self, rhs: i8) -> Word {
        Word(i32::from(self.0).wrapping_add(i32::from(rhs)) as u16)
    }
}

impl AddAssign<i8> for Word {
    fn add_assign(&mut self, rhs: i8) {
        self.0 = i32::from(self.0).wrapping_add(i32::from(rhs)) as u16;
    }
}

impl Add<i16> for Word {
    type Output = Word;

    fn add(self, rhs: i16) -> Word {
        Word(i32::from(self.0).wrapping_add(i32::from(rhs)) as u16)
    }
}

impl AddAssign<i16> for Word {
    fn add_assign(&mut self, rhs: i16) {
        self.0 = i32::from(self.0).wrapping_add(i32::from(rhs)) as u16;
    }
}

impl Add<i32> for Word {
    type Output = Word;

    fn add(self, rhs: i32) -> Word {
        Word(i32::from(self.0).wrapping_add(rhs) as u16)
    }
}

impl AddAssign<i32> for Word {
    fn add_assign(&mut self, rhs: i32) {
        self.0 = i32::from(self.0).wrapping_add(rhs) as u16;
    }
}

impl Sub<u16> for Word {
    type Output = Word;

    fn sub(self, rhs: u16) -> Word {
        Word(self.0.wrapping_sub(rhs))
    }
}

impl SubAssign<u16> for Word {
    fn sub_assign(&mut self, rhs: u16) {
        self.0 = self.0.wrapping_sub(rhs);
    }
}

impl Sub<Byte> for Word {
    type Output = Word;

    fn sub(self, rhs: Byte) -> Word {
        Word(self.0.wrapping_sub(u16::from(rhs)))
    }
}

impl SubAssign<Byte> for Word {
    fn sub_assign(&mut self, rhs: Byte) {
        self.0 = self.0.wrapping_sub(u16::from(rhs));
    }
}

impl Sub for Word {
    type Output = Word;

    fn sub(self, rhs: Word) -> Word {
        Word(self.0.wrapping_sub(rhs.0))
    }
}

impl Sub<i8> for Word {
    type Output = Word;

    fn sub(self, rhs: i8) -> Word {
        Word(i32::from(self.0).wrapping_sub(i32::from(rhs)) as u16)
    }
}

impl SubAssign<i8> for Word {
    fn sub_assign(&mut self, rhs: i8) {
        self.0 = i32::from(self.0).wrapping_sub(i32::from(rhs)) as u16;
    }
}

impl Sub<i16> for Word {
    type Output = Word;

    fn sub(self, rhs: i16) -> Word {
        Word(i32::from(self.0).wrapping_sub(i32::from(rhs)) as u16)
    }
}

impl SubAssign<i16> for Word {
    fn sub_assign(&mut self, rhs: i16) {
        self.0 = i32::from(self.0).wrapping_sub(i32::from(rhs)) as u16;
    }
}

impl Sub<i32> for Word {
    type Output = Word;

    fn sub(self, rhs: i32) -> Word {
        Word(i32::from(self.0).wrapping_sub(rhs) as u16)
    }
}

impl SubAssign<i32> for Word {
    fn sub_assign(&mut self, rhs: i32) {
        self.0 = i32::from(self.0).wrapping_sub(rhs) as u16;
    }
}

impl Shl<u16> for Word {
    type Output = Word;

    fn shl(self, rhs: u16) -> Word {
        Word(self.0 << rhs)
    }
}

impl ShlAssign<u16> for Word {
    fn shl_assign(&mut self, rhs: u16) {
        self.0 <<= rhs;
    }
}

impl Shr<u16> for Word {
    type Output = Word;

    fn shr(self, rhs: u16) -> Word {
        Word(self.0 >> rhs)
    }
}

impl ShrAssign<u16> for Word {
    fn shr_assign(&mut self, rhs: u16) {
        self.0 >>= rhs;
    }
}

impl BitAnd<u16> for Word {
    type Output = Word;

    fn bitand(self, rhs: u16) -> Word {
        Word(self.0 & rhs)
    }
}

impl BitAnd<Word> for Word {
    type Output = Word;

    fn bitand(self, rhs: Word) -> Word {
        Word(self.0 & rhs.0)
    }
}

impl BitAnd<Byte> for Word {
    type Output = Word;

    fn bitand(self, rhs: Byte) -> Word {
        Word(self.0 & u16::from(rhs))
    }
}

impl BitAnd<i32> for Word {
    type Output = Word;

    fn bitand(self, rhs: i32) -> Word {
        Word((i32::from(self.0) & rhs) as u16)
    }
}

impl BitAndAssign<u16> for Word {
    fn bitand_assign(&mut self, rhs: u16) {
        self.0 &= rhs;
    }
}

impl BitAndAssign<Word> for Word {
    fn bitand_assign(&mut self, rhs: Word) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<Byte> for Word {
    fn bitand_assign(&mut self, rhs: Byte) {
        self.0 &= u16::from(rhs);
    }
}

impl BitOr for Word {
    type Output = Word;

    fn bitor(self, rhs: Self) -> Word {
        Word(self.0 | rhs.0)
    }
}

impl BitOr<u16> for Word {
    type Output = Word;

    fn bitor(self, rhs: u16) -> Word {
        Word(self.0 | rhs)
    }
}

impl BitOr<Byte> for Word {
    type Output = Word;

    fn bitor(self, rhs: Byte) -> Word {
        Word(self.0 | u16::from(rhs))
    }
}

impl BitOrAssign<Word> for Word {
    fn bitor_assign(&mut self, rhs: Word) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<u16> for Word {
    fn bitor_assign(&mut self, rhs: u16) {
        self.0 |= rhs;
    }
}

impl BitOrAssign<Byte> for Word {
    fn bitor_assign(&mut self, rhs: Byte) {
        self.0 |= u16::from(rhs);
    }
}

impl BitXor for Word {
    type Output = Word;

    fn bitxor(self, rhs: Word) -> Word {
        Word(self.0 ^ rhs.0)
    }
}

impl BitXor<u16> for Word {
    type Output = Word;

    fn bitxor(self, rhs: u16) -> Word {
        Word(self.0 ^ rhs)
    }
}

impl BitXor<Byte> for Word {
    type Output = Word;

    fn bitxor(self, rhs: Byte) -> Word {
        Word(self.0 ^ u16::from(rhs))
    }
}

impl BitXorAssign<Word> for Word {
    fn bitxor_assign(&mut self, rhs: Word) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<u16> for Word {
    fn bitxor_assign(&mut self, rhs: u16) {
        self.0 ^= rhs;
    }
}

impl BitXorAssign<Byte> for Word {
    fn bitxor_assign(&mut self, rhs: Byte) {
        self.0 ^= u16::from(rhs);
    }
}

impl Not for Word {
    type Output = Word;

    fn not(self) -> Word {
        Word(!self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Word;
    use crate::Byte;

    #[test]
    fn wrapping_add_with_u16() {
        let result = Word::from(0xFFFEu16) + 0x0005u16;
        assert_eq!(result, Word::from(0x0003u16));
    }

    #[test]
    fn wrapping_add_with_negative_literal() {
        let result = Word::from(10u16) + -400i32;
        assert_eq!(result, Word(65146u16));
    }

    #[test]
    fn wrapping_sub_with_u16() {
        let result = Word::from(0x0003u16) - 0x0004u16;
        assert_eq!(result, Word(0xFFFF));
    }

    #[test]
    fn bitwise_helpers_cover_newtypes() {
        let mut word = Word(0x0F0F);
        word &= Word(0x00FF);
        assert_eq!(word, Word(0x000F));

        word |= Byte(0xF0);
        assert_eq!(word, Word(0x00FF));

        word ^= 0x0FF0u16;
        assert_eq!(word, Word(0x0F0F));

        assert_eq!(!word, Word(0xF0F0));
    }
}
