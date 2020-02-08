use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Div, Mul, Shl, Shr, Sub};

use super::{Bit, ParseResult};

/// Binary: Sequence of Bits, ordered from most to least significant
pub struct Binary([Bit; 64]);

impl Binary {
    /// Create a Binary filled with provided Bit
    pub fn of(b: Bit) -> Binary {
        Binary([b; 64])
    }

    /// Create a Binary representing 0
    pub fn zero() -> Binary {
        Binary::of(Bit::Off)
    }

    /// Create a Binary representing 1
    pub fn one() -> Binary {
        let mut binary = Binary::zero();
        binary.set(63, Bit::On);
        binary
    }

    /// Create a Binary from an int
    pub fn from_int(n: i64) -> Binary {
        // Need 66 chars to represent 64bit, since it adds "0b" to beginning
        let bit_string = format!("{:#066b}", n);
        let mut binary = Binary::zero();

        bit_string
            .chars()
            .skip(2)
            .map(|c| if c == '1' { Bit::On } else { Bit::Off })
            .enumerate()
            .for_each(|(i, bit)| binary.set(i, bit));

        binary
    }

    /// Attempt to convert Binary to an int
    pub fn to_int(self) -> ParseResult {
        // Rust is a little inconsistent with how it handles negative binary numbers...
        //
        //   - Literals use negative, eg. -0b0000_0011 for -3)
        //   - format! outputs two's complement, eg. "0b1111_1101" for `format!("{:#010b}", -3)`
        //   - i8::from_str_radix expects a stringified version of a negative literal, rather than
        //      two's complement
        //
        // ... which means we can't simply format! -> build complement string w/ logic -> parse with
        // from_str_radix. If negative, need to convert from two's complement to negative literal

        // Capacity should fit full number and potentially "-" sign
        let mut s = String::with_capacity(65);
        let mut binary = self;

        if binary.is_negative() {
            // Take complement to make "positive" and then add sign
            s.push('-');
            binary = binary.complement();
        }

        for i in 0..64 {
            s.push(if binary.is_on_at(i) { '1' } else { '0' });
        }

        i64::from_str_radix(s.as_str(), 2)
    }

    /// Returns Bit at given position
    pub fn get(&self, i: usize) -> Bit {
        self.0[i]
    }

    /// Sets Bit at given position
    pub fn set(&mut self, i: usize, b: Bit) {
        self.0[i] = b;
    }

    /// Returns whether or not Bit at given position is on
    pub fn is_on_at(&self, i: usize) -> bool {
        match self.0[i] {
            Bit::On => true,
            _ => false,
        }
    }

    /// Returns whether or not Binary represents negative number
    pub fn is_negative(&self) -> bool {
        self.is_on_at(0)
    }

    /// Returns ownership for a new copy of self
    pub fn copy(&self) -> Binary {
        let mut copy = Binary::zero();

        for i in 0..64 {
            copy.set(i, self.get(i));
        }

        copy
    }

    /// Inverts the sign of a Binary by flipping bits and adding 1
    pub fn complement(&self) -> Binary {
        let mut comp = Binary::zero();

        for i in 0..64 {
            comp.set(i, !self.get(i));
        }

        &comp + &Binary::one()
    }
}

// FIXME this and to_int should share some codez?
impl fmt::Debug for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0b")?;

        for i in 0..64 {
            if i % 4 == 0 {
                write!(f, "_")?;
            }
            self.get(i).fmt(f)?
        }
        Ok(())
    }
}

impl PartialEq for Binary {
    fn eq(&self, other: &Binary) -> bool {
        for i in 0..64 {
            if self.get(i) != other.get(i) {
                return false;
            }
        }

        return true;
    }
}

impl Shl<usize> for &Binary {
    type Output = Binary;

    fn shl(self, rhs: usize) -> Binary {
        let mut shifted = Binary::zero();

        for i in 0..(64 - rhs) {
            shifted.set(i, self.get(i + 1));
        }

        shifted
    }
}

impl Shr<usize> for &Binary {
    type Output = Binary;

    fn shr(self, rhs: usize) -> Binary {
        let mut shifted = Binary::zero();

        for i in rhs..64 {
            shifted.set(i, self.get(i - 1));
        }

        shifted
    }
}

impl<'a, 'b> Add<&'b Binary> for &'a Binary {
    type Output = Binary;

    /// Basic implementation of full addition circuit
    fn add(self, other: &'b Binary) -> Binary {
        let mut rval = Binary::zero();
        let mut carry = Bit::Off;

        for i in (0..64).rev() {
            let result = match i {
                63 => Bit::half_adder(self.get(i), other.get(i)),
                _ => Bit::full_adder(self.get(i), other.get(i), carry),
            };

            rval.set(i, result.0);
            carry = result.1;
        }

        rval
    }
}

impl<'a, 'b> Sub<&'b Binary> for &'a Binary {
    type Output = Binary;

    /// Simple subtractor that takes complement of other and adds to self
    fn sub(self, other: &'b Binary) -> Binary {
        self + &other.complement()
    }
}

impl<'a, 'b> Mul<&'b Binary> for &'a Binary {
    type Output = Binary;

    /// Multiplier that uses basic series of shifts and adding partial products
    fn mul(self, other: &'b Binary) -> Binary {
        let mut accumulator = Binary::zero();

        // For each Bit in the multiplier, starting at least significant...
        for i in (0..64).rev() {
            let multiplier = other.get(i);

            // ... create a zeroed Binary to hold the partial product...
            let mut partial = Binary::zero();

            // ... and then iterate through each Bit in multiplicand,
            // starting with least significant (truncating off more
            // significant Bits as necessary due to storage size)...
            for j in ((63 - i)..64).rev() {
                let product = Bit::multiplier(self.get(j), multiplier);

                // ... and with each multiplier, the index for where the least
                // significant Bit gets copied into the partial shifts more and
                // more to the left...
                partial.set(j - (63 - i), product);
            }

            accumulator = &accumulator + &partial;
        }

        accumulator
    }
}

impl<'a, 'b> Div<&'b Binary> for &'a Binary {
    type Output = Binary;

    /// Emulates long division in a comically long fashion
    fn div(self, other: &'b Binary) -> Binary {
        // Take "absolute value" of the binaries for simpler math,
        // storing whether final quotient should be negative
        let negate = self.is_negative() != other.is_negative();

        let dividend = if self.is_negative() {
            self.complement()
        } else {
            self.copy()
        };

        let divisor = if other.is_negative() {
            other.complement()
        } else {
            other.copy()
        };

        let mut quotient = Binary::zero();

        // The partial dividend starts at "zero", with each successive round
        // shifting the next bit from the original number
        let mut partial_dividend = Binary::zero();

        for i in 0..64 {
            // Shift partial left and assign next bit to least significant
            partial_dividend = &partial_dividend << 1;
            partial_dividend.set(63, dividend.get(i));

            // Now go bit by bit and check if either is greater and, if so,
            // breaking the loop and determining what value to assign to quot
            let mut result = Bit::On;

            for j in 0..64 {
                if partial_dividend.get(j) != divisor.get(j) {
                    if divisor.get(j) == Bit::On {
                        result = Bit::Off;
                    }

                    break;
                }
            }

            quotient.set(i, result);

            // If the result was a "1" then we need to create a new dividend by
            // subtracting the divisor from the prior dividend
            if result == Bit::On {
                partial_dividend = &partial_dividend - &divisor;
            }
        }

        if negate {
            quotient.complement()
        } else {
            quotient
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Binary, Bit};

    #[test]
    fn test_shl() {
        let one = Binary::one();
        let mut two = Binary::zero();
        two.set(62, Bit::On);

        let shifted = &one << 1;

        assert_ne!(one, two);
        assert_eq!(shifted, two);
    }

    #[test]
    fn test_shr() {
        let one = Binary::one();
        let mut two = Binary::zero();
        two.set(62, Bit::On);

        let shifted = &two >> 1;

        assert_eq!(shifted, one);
    }

    #[test]
    fn test_partial_eq() {
        assert_ne!(Binary::zero(), Binary::one());
    }

    #[test]
    fn from_int_test() {
        let zero = Binary::zero();
        let one = Binary::one();
        let negative_one = Binary::of(Bit::On);

        let min = {
            let mut ba = Binary::of(Bit::Off);
            ba.set(0, Bit::On);
            ba
        };

        let max = {
            let mut ba = Binary::of(Bit::On);
            ba.set(0, Bit::Off);
            ba
        };

        assert_eq!(Binary::from_int(0), zero, "0");
        assert_eq!(Binary::from_int(1), one, "1");
        assert_eq!(Binary::from_int(-1), negative_one, "-1");
        assert_eq!(Binary::from_int(-9223372036854775808), min, "min");
        assert_eq!(Binary::from_int(9223372036854775807), max, "max");
    }

    #[test]
    fn to_int_test() {
        let zero = Binary::zero();
        let one = Binary::one();
        let negative_one = Binary::of(Bit::On);

        let min = {
            let mut ba = Binary::of(Bit::Off);
            ba.set(0, Bit::On);
            ba
        };

        let max = {
            let mut ba = Binary::of(Bit::On);
            ba.set(0, Bit::Off);
            ba
        };

        assert_eq!(Binary::to_int(zero), Ok(0), "0");
        assert_eq!(Binary::to_int(one), Ok(1), "1");
        assert_eq!(Binary::to_int(negative_one), Ok(-1), "-1");
        assert_eq!(Binary::to_int(min), Ok(::std::i64::MIN), "min");
        assert_eq!(Binary::to_int(max), Ok(::std::i64::MAX), "max");
    }
}
