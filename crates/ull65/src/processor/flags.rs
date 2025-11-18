//! Processor status register (P) flags.
//!
//! The 6502 status register contains 8 condition code and control bits:
//! Carry, Zero, Interrupt Disable, Decimal Mode, Break, Expansion, Overflow, and Sign.

use bitflags::bitflags;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use ull::Byte;

bitflags! {
    /// Processor status register (P register).
    ///
    /// Each bit represents a different condition code or control flag. Many instruction
    /// automatically update these flags based on their results.
    ///
    /// # Bit Layout
    ///
    /// ```text
    /// 7 6 5 4 3 2 1 0
    /// N V E B D I Z C
    /// ```
    ///
    /// - N (Negative): Set when result has bit 7 set to represent a signed number
    /// - V (Overflow): Set on signed overflow
    /// - E (Expansion): Unused, always 1
    /// - B (Break): Distinguishes BRK from IRQ
    /// - D (Decimal): Enables BCD mode
    /// - I (Interrupt Disable): Masks IRQ when set
    /// - Z (Zero): Set when result is zero
    /// - C (Carry): Set on carry/borrow
    #[derive(Default, Copy, Clone, Debug)]
    pub struct Flags: u8 {
        const Carry = 0b0000_0001;              // C
        const Zero = 0b0000_0010;               // Z
        const InterruptDisabled = 0b0000_0100;  // I
        const DecimalMode = 0b0000_1000;        // D
        const Break = 0b0001_0000;              // B
        const Expansion = 0b0010_0000;          // E
        const Overflow = 0b0100_0000;           // V
        const Sign = 0b1000_0000;               // N
    }
}

impl Flags {
    #[inline]
    pub fn set_zero(&mut self, zero: bool) {
        self.set(Flags::Zero, zero);
    }

    #[inline]
    pub fn set_signed(&mut self, signed: bool) {
        self.set(Flags::Sign, signed);
    }

    #[inline]
    pub fn set_carry(&mut self, carry: bool) {
        self.set(Flags::Carry, carry);
    }

    #[inline]
    pub fn set_overflow(&mut self, overflow: bool) {
        self.set(Flags::Overflow, overflow);
    }

    #[inline]
    pub fn set_decimal_mode(&mut self, enabled: bool) {
        self.set(Flags::DecimalMode, enabled);
    }

    #[inline]
    pub fn set_interrupt_disabled(&mut self, disabled: bool) {
        self.set(Flags::InterruptDisabled, disabled);
    }

    #[must_use]
    #[inline]
    pub fn bit(self, flag: Flags) -> u8 {
        u8::from(self.contains(flag))
    }
}

impl From<Byte> for Flags {
    fn from(byte: Byte) -> Self {
        Flags::from_bits_truncate(u8::from(byte))
    }
}

impl From<Flags> for Byte {
    fn from(flags: Flags) -> Self {
        Byte(flags.bits())
    }
}

impl PartialEq<Flags> for Byte {
    fn eq(&self, other: &Flags) -> bool {
        self.0 == other.bits()
    }
}

impl PartialEq<Byte> for Flags {
    fn eq(&self, other: &Byte) -> bool {
        self.bits() == other.0
    }
}

impl BitAnd<Flags> for Byte {
    type Output = Byte;

    fn bitand(self, rhs: Flags) -> Byte {
        Byte(self.0 & rhs.bits())
    }
}

impl BitAndAssign<Flags> for Byte {
    fn bitand_assign(&mut self, rhs: Flags) {
        self.0 &= rhs.bits();
    }
}

impl BitOr<Flags> for Byte {
    type Output = Byte;

    fn bitor(self, rhs: Flags) -> Byte {
        Byte(self.0 | rhs.bits())
    }
}

impl BitOrAssign<Flags> for Byte {
    fn bitor_assign(&mut self, rhs: Flags) {
        self.0 |= rhs.bits();
    }
}

impl BitXor<Flags> for Byte {
    type Output = Byte;

    fn bitxor(self, rhs: Flags) -> Byte {
        Byte(self.0 ^ rhs.bits())
    }
}

impl BitXorAssign<Flags> for Byte {
    fn bitxor_assign(&mut self, rhs: Flags) {
        self.0 ^= rhs.bits();
    }
}
