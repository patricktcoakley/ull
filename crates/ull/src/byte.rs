//! Type-safe 8-bit value with automatic wrapping arithmetic.
//!
//! # Examples
//!
//! ```
//! use ull::Byte;
//!
//! // Addition wraps like 6502 hardware
//! assert_eq!(Byte(0xFF) + 1, Byte(0x00));
//!
//! // Works with u8 literals
//! assert_eq!(Byte(0xFF) + 1u8, Byte(0x00));
//! ```

use crate::{Address, nibble::Nibble};
use core::fmt::{Display, Formatter, LowerHex, UpperHex};
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// Type-safe 8-bit value with wrapping arithmetic.
///
/// Wraps a `u8` and provides operator overloads that automatically wrap on overflow,
/// matching 6502 hardware behavior. Use the [`byte!`](crate::byte!) macro for convenient construction.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Byte(pub u8);

#[macro_export]
macro_rules! byte {
    ($val:expr) => {
        $crate::byte::Byte::from($val)
    };
}

impl Byte {
    pub const ZERO: Byte = Byte(0);
    pub const MAX: Byte = Byte(0xFF);

    /// Returns `true` if bit 7 is set (negative in signed arithmetic).
    #[inline]
    #[must_use]
    pub fn is_signed(self) -> bool {
        self.0 & 0x80 != 0
    }

    #[inline]
    #[must_use]
    pub fn lo(self) -> Nibble {
        Nibble(self.0 & 0x0F)
    }

    #[inline]
    #[must_use]
    pub fn hi(self) -> Nibble {
        Nibble((self.0 >> 4) & 0x0F)
    }
}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl LowerHex for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Byte(value)
    }
}
impl From<bool> for Byte {
    fn from(value: bool) -> Self {
        Byte(value as u8)
    }
}
impl From<i8> for Byte {
    fn from(value: i8) -> Self {
        Byte(value as u8)
    }
}
impl From<u16> for Byte {
    fn from(value: u16) -> Self {
        Byte(value as u8)
    }
}
impl From<i16> for Byte {
    fn from(value: i16) -> Self {
        Byte(value as u8)
    }
}
impl From<u32> for Byte {
    fn from(value: u32) -> Self {
        Byte(value as u8)
    }
}
impl From<i32> for Byte {
    fn from(value: i32) -> Self {
        Byte(value as u8)
    }
}

impl From<(u8, u8)> for Byte {
    fn from((low, high): (u8, u8)) -> Self {
        Byte(((high & 0x0F) << 4) | (low & 0x0F))
    }
}

impl From<(Nibble, Nibble)> for Byte {
    fn from((low, high): (Nibble, Nibble)) -> Self {
        Byte::from((low.as_u8(), high.as_u8()))
    }
}

impl From<Nibble> for Byte {
    fn from(value: Nibble) -> Self {
        Byte(value.as_u8())
    }
}

impl From<Byte> for u8 {
    fn from(value: Byte) -> Self {
        value.0
    }
}
impl From<Byte> for i8 {
    fn from(value: Byte) -> Self {
        value.0 as i8
    }
}
impl From<Byte> for u16 {
    fn from(value: Byte) -> Self {
        u16::from(value.0)
    }
}
impl From<Byte> for i16 {
    fn from(value: Byte) -> Self {
        i16::from(value.0)
    }
}
impl From<Byte> for u32 {
    fn from(value: Byte) -> Self {
        u32::from(value.0)
    }
}
impl From<Byte> for i32 {
    fn from(value: Byte) -> Self {
        i32::from(value.0)
    }
}
impl From<Byte> for usize {
    fn from(value: Byte) -> Self {
        value.0 as usize
    }
}

impl From<usize> for Byte {
    fn from(value: usize) -> Self {
        Byte((value & 0xFF) as u8)
    }
}

impl Add<Byte> for Byte {
    type Output = Byte;

    fn add(self, rhs: Byte) -> Byte {
        Byte(self.0.wrapping_add(rhs.0))
    }
}

impl Add<u8> for Byte {
    type Output = Byte;

    fn add(self, rhs: u8) -> Byte {
        Byte(self.0.wrapping_add(rhs))
    }
}

impl Add<usize> for Byte {
    type Output = Byte;

    fn add(self, rhs: usize) -> Byte {
        Byte(self.0.wrapping_add(rhs as u8))
    }
}

impl Add<i32> for Byte {
    type Output = Byte;

    fn add(self, rhs: i32) -> Byte {
        byte!(i32::from(self.0).wrapping_add(rhs))
    }
}

impl AddAssign<i32> for Byte {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl AddAssign<Byte> for Byte {
    fn add_assign(&mut self, rhs: Byte) {
        self.0 = self.0.wrapping_add(rhs.0);
    }
}

impl AddAssign<u8> for Byte {
    fn add_assign(&mut self, rhs: u8) {
        self.0 = self.0.wrapping_add(rhs);
    }
}

impl Sub<Byte> for Byte {
    type Output = Byte;

    fn sub(self, rhs: Byte) -> Byte {
        Byte(self.0.wrapping_sub(rhs.0))
    }
}

impl Sub<u8> for Byte {
    type Output = Byte;

    fn sub(self, rhs: u8) -> Byte {
        Byte(self.0.wrapping_sub(rhs))
    }
}

impl Sub<usize> for Byte {
    type Output = Byte;

    fn sub(self, rhs: usize) -> Byte {
        Byte(self.0.wrapping_sub(rhs as u8))
    }
}

impl Sub<i32> for Byte {
    type Output = Byte;

    fn sub(self, rhs: i32) -> Byte {
        byte!(i32::from(self.0).wrapping_sub(rhs))
    }
}

impl SubAssign<i32> for Byte {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}

impl SubAssign<Byte> for Byte {
    fn sub_assign(&mut self, rhs: Byte) {
        self.0 = self.0.wrapping_sub(rhs.0);
    }
}

impl SubAssign<u8> for Byte {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 = self.0.wrapping_sub(rhs);
    }
}

impl BitAnd<Byte> for Byte {
    type Output = Byte;

    fn bitand(self, rhs: Byte) -> Byte {
        Byte(self.0 & rhs.0)
    }
}

impl BitAnd<u8> for Byte {
    type Output = Byte;

    fn bitand(self, rhs: u8) -> Byte {
        Byte(self.0 & rhs)
    }
}

impl BitAndAssign<Byte> for Byte {
    fn bitand_assign(&mut self, rhs: Byte) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<u8> for Byte {
    fn bitand_assign(&mut self, rhs: u8) {
        self.0 &= rhs;
    }
}

impl BitOr<Byte> for Byte {
    type Output = Byte;

    fn bitor(self, rhs: Byte) -> Byte {
        Byte(self.0 | rhs.0)
    }
}

impl BitOr<u8> for Byte {
    type Output = Byte;

    fn bitor(self, rhs: u8) -> Byte {
        Byte(self.0 | rhs)
    }
}

impl BitOrAssign<Byte> for Byte {
    fn bitor_assign(&mut self, rhs: Byte) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<u8> for Byte {
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= rhs;
    }
}

impl Add<Nibble> for Byte {
    type Output = Byte;

    fn add(self, rhs: Nibble) -> Byte {
        self + rhs.as_u8()
    }
}

impl AddAssign<Nibble> for Byte {
    fn add_assign(&mut self, rhs: Nibble) {
        *self = *self + rhs;
    }
}

impl Sub<Nibble> for Byte {
    type Output = Byte;

    fn sub(self, rhs: Nibble) -> Byte {
        self - rhs.as_u8()
    }
}

impl SubAssign<Nibble> for Byte {
    fn sub_assign(&mut self, rhs: Nibble) {
        *self = *self - rhs;
    }
}

impl BitAnd<Nibble> for Byte {
    type Output = Byte;

    fn bitand(self, rhs: Nibble) -> Byte {
        self & rhs.as_u8()
    }
}

impl BitAndAssign<Nibble> for Byte {
    fn bitand_assign(&mut self, rhs: Nibble) {
        *self &= rhs.as_u8();
    }
}

impl BitOr<Nibble> for Byte {
    type Output = Byte;

    fn bitor(self, rhs: Nibble) -> Byte {
        self | rhs.as_u8()
    }
}

impl BitOrAssign<Nibble> for Byte {
    fn bitor_assign(&mut self, rhs: Nibble) {
        *self |= rhs.as_u8();
    }
}

impl BitXor<Nibble> for Byte {
    type Output = Byte;

    fn bitxor(self, rhs: Nibble) -> Byte {
        self ^ rhs.as_u8()
    }
}

impl BitXorAssign<Nibble> for Byte {
    fn bitxor_assign(&mut self, rhs: Nibble) {
        *self ^= rhs.as_u8();
    }
}

impl BitXor<Byte> for Byte {
    type Output = Byte;

    fn bitxor(self, rhs: Byte) -> Byte {
        Byte(self.0 ^ rhs.0)
    }
}

impl BitXor<u8> for Byte {
    type Output = Byte;

    fn bitxor(self, rhs: u8) -> Byte {
        Byte(self.0 ^ rhs)
    }
}

impl BitXorAssign<Byte> for Byte {
    fn bitxor_assign(&mut self, rhs: Byte) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<u8> for Byte {
    fn bitxor_assign(&mut self, rhs: u8) {
        self.0 ^= rhs;
    }
}

impl Not for Byte {
    type Output = Byte;

    fn not(self) -> Byte {
        Byte(!self.0)
    }
}

impl Shl<u8> for Byte {
    type Output = Byte;

    fn shl(self, rhs: u8) -> Byte {
        Byte(self.0.wrapping_shl(u32::from(rhs)))
    }
}

impl ShlAssign<u8> for Byte {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 = self.0.wrapping_shl(u32::from(rhs));
    }
}

impl Shr<u8> for Byte {
    type Output = Byte;

    fn shr(self, rhs: u8) -> Byte {
        Byte(self.0.wrapping_shr(u32::from(rhs)))
    }
}

impl ShrAssign<u8> for Byte {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 = self.0.wrapping_shr(u32::from(rhs));
    }
}

impl PartialEq<u8> for Byte {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Byte> for u8 {
    fn eq(&self, other: &Byte) -> bool {
        *self == other.0
    }
}

impl PartialOrd<u8> for Byte {
    fn partial_cmp(&self, other: &u8) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Byte> for u8 {
    fn partial_cmp(&self, other: &Byte) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Byte;

    #[test]
    fn wrapping_add_wraps() {
        let result = Byte(0xFF) + 1u8;
        assert_eq!(result, Byte(0x00));
    }

    #[test]
    fn wrapping_sub_wraps() {
        let result = Byte(0x00) - 1u8;
        assert_eq!(result, Byte(0xFF));
    }

    #[test]
    fn overflowing_sub_reports_borrow() {
        let (raw, borrow) = Byte(0x10).0.overflowing_sub(0x20);
        assert_eq!(Byte(raw), Byte(0xF0));
        assert!(borrow);
    }
}
