use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// Bit: Representation of a transistor, either on or off
#[derive(Copy, Clone, PartialEq)]
pub enum Bit {
    Off,
    On,
}

use self::Bit::*;

impl Bit {
    /// Single-bit adder circuit for two inputs
    pub fn half_adder(b1: Bit, b2: Bit) -> (Bit, Bit) {
        (b1 ^ b2, b1 & b2)
    }

    /// Single-bit adder circuit for three inputs
    pub fn full_adder(b1: Bit, b2: Bit, b3: Bit) -> (Bit, Bit) {
        let (sum1, carry1) = Bit::half_adder(b1, b2);
        let (sum2, carry2) = Bit::half_adder(sum1, b3);

        (sum2, carry1 | carry2)
    }

    /// Single-bit multiplier circuit
    pub fn multiplier(b1: Bit, b2: Bit) -> Bit {
        b1 & b2
    }
}

impl fmt::Debug for Bit {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Off => 0.fmt(formatter),
            On => 1.fmt(formatter),
        }
    }
}

impl PartialOrd for Bit {
    fn partial_cmp(&self, other: &Bit) -> Option<Ordering> {
        Some(if self == other {
            Ordering::Equal
        } else if *self == Bit::On && *other == Bit::Off {
            Ordering::Greater
        } else {
            Ordering::Less
        })
    }
}

impl BitAnd for Bit {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        match (self, other) {
            (On, On) => On,
            _ => Off,
        }
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (On, _) | (_, On) => On,
            _ => Off,
        }
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        if self == other {
            Off
        } else {
            On
        }
    }
}

impl Not for Bit {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Off => On,
            On => Off,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bit;
    use super::Bit::{Off, On};

    #[test]
    fn test_half_adder() {
        assert_eq!(Bit::half_adder(Off, Off), (Off, Off));
        assert_eq!(Bit::half_adder(On, Off), (On, Off));
        assert_eq!(Bit::half_adder(Off, On), (On, Off));
        assert_eq!(Bit::half_adder(On, On), (Off, On));
    }

    #[test]
    fn test_full_adder() {
        // No sum, no carry
        assert_eq!(Bit::full_adder(Off, Off, Off), (Off, Off));

        // Sum, no carry
        assert_eq!(Bit::full_adder(On, Off, Off), (On, Off));
        assert_eq!(Bit::full_adder(Off, On, Off), (On, Off));
        assert_eq!(Bit::full_adder(Off, Off, On), (On, Off));

        // Carry, no sum
        assert_eq!(Bit::full_adder(On, On, Off), (Off, On));
        assert_eq!(Bit::full_adder(Off, On, On), (Off, On));
        assert_eq!(Bit::full_adder(On, Off, On), (Off, On));

        // Sum, carry
        assert_eq!(Bit::full_adder(On, On, On), (On, On));
    }

    #[test]
    fn test_multiplier() {
        assert_eq!(Bit::multiplier(Off, Off), Off);
        assert_eq!(Bit::multiplier(Off, On), Off);
        assert_eq!(Bit::multiplier(On, Off), Off);
        assert_eq!(Bit::multiplier(On, On), On);
    }
}
