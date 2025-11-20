//! Marker trait for values that can act as memory addresses.
//!
//! Implementers expose a uniform way to index (`as_usize`) plus wrapping addition
//! and subtraction so buses can walk memory without caring about the underlying
//! width (zero-page `Byte`, full `Word`, or even plain integers).

use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{Byte, Nibble, Word};

pub trait Address:
    Copy + Add<usize, Output = Self> + Sub<usize, Output = Self> + AddAssign<usize> + SubAssign<usize>
{
    /// Returns the address as a host `usize` for indexing.
    fn as_usize(self) -> usize;

    /// Returns the address truncated to 16 bits.
    #[inline]
    fn as_u16(self) -> u16 {
        self.as_usize() as u16
    }

    /// Returns the address truncated to 8 bits.
    #[inline]
    fn as_u8(self) -> u8 {
        self.as_usize() as u8
    }
}

impl Address for Byte {
    #[inline]
    fn as_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    fn as_u16(self) -> u16 {
        self.0 as u16
    }

    #[inline]
    fn as_u8(self) -> u8 {
        self.0
    }
}

impl AddAssign<usize> for Byte {
    fn add_assign(&mut self, rhs: usize) {
        self.0 = self.0.wrapping_add(rhs as u8);
    }
}

impl SubAssign<usize> for Byte {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 = self.0.wrapping_sub(rhs as u8);
    }
}

impl Address for Nibble {
    #[inline]
    fn as_usize(self) -> usize {
        usize::from(self.0)
    }

    #[inline]
    fn as_u16(self) -> u16 {
        u16::from(self.0)
    }

    #[inline]
    fn as_u8(self) -> u8 {
        self.0
    }
}

impl AddAssign<usize> for Nibble {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl SubAssign<usize> for Nibble {
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs;
    }
}

impl Address for Word {
    #[inline]
    fn as_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    fn as_u16(self) -> u16 {
        self.0
    }

    #[inline]
    fn as_u8(self) -> u8 {
        (self.0 & 0x00FF) as u8
    }
}

impl AddAssign<usize> for Word {
    fn add_assign(&mut self, rhs: usize) {
        self.0 = self.0.wrapping_add(rhs as u16);
    }
}

impl SubAssign<usize> for Word {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 = self.0.wrapping_sub(rhs as u16);
    }
}

impl Address for usize {
    #[inline]
    fn as_usize(self) -> usize {
        self
    }

    #[inline]
    fn as_u16(self) -> u16 {
        self as u16
    }

    #[inline]
    fn as_u8(self) -> u8 {
        self as u8
    }
}
