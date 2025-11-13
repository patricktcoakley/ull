//! 4-bit nibble type for internal BCD arithmetic.
//!
//! The `Nibble` type represents a 4-bit value (0x0-0xF), used internally for
//! Binary Coded Decimal (BCD) operations in decimal mode. All operations automatically
//! mask to the lower 4 bits.
//!

use crate::byte::Byte;
use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, AddAssign, BitAnd, BitOr, BitXor, Not, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nibble(pub u8);

#[macro_export]
macro_rules! nibble {
    ($val:expr) => {
        $crate::nibble::Nibble::from($val)
    };
}

impl Nibble {
    pub const ZERO: Nibble = Nibble(0);
    pub const MAX: u8 = 0x0F;

    #[inline]
    fn mask(value: u8) -> u8 {
        value & Self::MAX
    }

    /// Converts to `usize` for indexing.
    #[inline]
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<u8> for Nibble {
    fn from(value: u8) -> Self {
        debug_assert!(value <= Self::MAX, "nibble out of range");
        Nibble(value & Self::MAX)
    }
}

impl From<Nibble> for u8 {
    fn from(value: Nibble) -> Self {
        value.0
    }
}

impl From<Nibble> for Byte {
    fn from(value: Nibble) -> Self {
        Byte::from(value.0)
    }
}

impl From<Nibble> for u16 {
    fn from(value: Nibble) -> Self {
        u16::from(value.0)
    }
}

impl From<Nibble> for i16 {
    fn from(value: Nibble) -> Self {
        i16::from(value.0)
    }
}

impl From<Nibble> for usize {
    fn from(value: Nibble) -> Self {
        usize::from(value.0)
    }
}

impl Add<Nibble> for Nibble {
    type Output = Nibble;

    fn add(self, rhs: Nibble) -> Nibble {
        Nibble::from(Self::mask(self.0 + rhs.0))
    }
}

impl Add<u8> for Nibble {
    type Output = Nibble;

    fn add(self, rhs: u8) -> Nibble {
        Nibble::from(Self::mask(self.0 + rhs))
    }
}

impl AddAssign<Nibble> for Nibble {
    fn add_assign(&mut self, rhs: Nibble) {
        *self = *self + rhs;
    }
}

impl AddAssign<u8> for Nibble {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl Sub<Nibble> for Nibble {
    type Output = Nibble;

    fn sub(self, rhs: Nibble) -> Nibble {
        Nibble::from(Self::mask(self.0.wrapping_sub(rhs.0)))
    }
}

impl Sub<u8> for Nibble {
    type Output = Nibble;

    fn sub(self, rhs: u8) -> Nibble {
        Nibble::from(Self::mask(self.0.wrapping_sub(rhs)))
    }
}

impl SubAssign<Nibble> for Nibble {
    fn sub_assign(&mut self, rhs: Nibble) {
        *self = *self - rhs;
    }
}

impl SubAssign<u8> for Nibble {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl BitAnd<Nibble> for Nibble {
    type Output = Nibble;

    fn bitand(self, rhs: Nibble) -> Nibble {
        Nibble::from(self.0 & rhs.0)
    }
}

impl BitOr<Nibble> for Nibble {
    type Output = Nibble;

    fn bitor(self, rhs: Nibble) -> Nibble {
        Nibble::from(self.0 | rhs.0)
    }
}

impl BitXor<Nibble> for Nibble {
    type Output = Nibble;

    fn bitxor(self, rhs: Nibble) -> Nibble {
        Nibble::from(self.0 ^ rhs.0)
    }
}

impl Not for Nibble {
    type Output = Nibble;

    fn not(self) -> Nibble {
        Nibble::from(!self.0)
    }
}

impl fmt::LowerHex for Nibble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for Nibble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl PartialEq<u8> for Nibble {
    fn eq(&self, other: &u8) -> bool {
        self.0 == (*other & Self::MAX)
    }
}

impl PartialEq<Nibble> for u8 {
    fn eq(&self, other: &Nibble) -> bool {
        other == self
    }
}

impl PartialOrd<u8> for Nibble {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        Some(self.0.cmp(&(*other & Self::MAX)))
    }
}

impl PartialOrd<Nibble> for u8 {
    fn partial_cmp(&self, other: &Nibble) -> Option<Ordering> {
        Some((*self & Nibble::MAX).cmp(&other.0))
    }
}

#[cfg(test)]
mod tests {
    use super::Nibble;

    #[test]
    fn add_wraps_to_low_four_bits() {
        assert_eq!(Nibble::from(0x0D) + 0x05, Nibble::from(0x02));
    }

    #[test]
    fn sub_wraps_to_low_four_bits() {
        assert_eq!(Nibble::from(0x01) - 0x03, Nibble::from(0x0E));
    }
}
