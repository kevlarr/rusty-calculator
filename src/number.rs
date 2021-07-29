use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Shl, Shr, Sub};

use crate::{Bit, ParseResult};

/// A representation of a 64-bit integer using a big-endian series of Bits
#[derive(Copy, Clone)]
pub struct Number([Bit; 64]);

impl Number {
    /// Create a Number filled with provided Bit
    pub fn from(b: Bit) -> Self {
        Self([b; 64])
    }

    /// Create a Number representing 0
    pub fn zero() -> Self {
        Self::from(Bit::Off)
    }

    /// Create a Number representing 1
    pub fn one() -> Self {
        let mut n = Self::zero();
        n.set(63, Bit::On);
        n
    }

    /// Create a Number from an int
    pub fn from_int(n: i64) -> Self {
        // Need 66 chars to represent 64-bit, since it adds "0b" to beginning
        //
        // TODO Would it be more appropriate for the project to manually perform
        // this conversion rather than rely on rust generating the bit string?
        let bit_string = format!("{:#066b}", n);
        let mut n = Self::zero();

        bit_string
            .chars()
            .skip(2)
            .map(|c| if c == '1' { Bit::On } else { Bit::Off })
            .enumerate()
            .for_each(|(i, bit)| n.set(i, bit));

        n
    }

    /// Attempt to convert Number to an int
    pub fn to_int(self) -> ParseResult {
        // Rust seems a little inconsistent with how it handles negative binary numbers...
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
        let mut n = self;

        if n.is_negative() {
            // Take complement to make "positive" and then add sign
            s.push('-');
            n = -n;
        }

        for i in 0..64 {
            s.push(if n.is_on_at(i) { '1' } else { '0' });
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
        self.0[i] == Bit::On
    }

    /// Returns whether or not Number represents negative number
    pub fn is_negative(&self) -> bool {
        self.is_on_at(0)
    }
}

// FIXME this and to_int should share some codez?
impl fmt::Debug for Number {
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

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        for i in 0..64 {
            if self.get(i) != other.get(i) {
                return false;
            }
        }

        return true;
    }
}

impl Shl<usize> for Number {
    type Output = Number;

    fn shl(self, rhs: usize) -> Number {
        let mut shifted = Number::zero();

        for i in 0..(64 - rhs) {
            shifted.set(i, self.get(i + 1));
        }

        shifted
    }
}

impl Shr<usize> for Number {
    type Output = Number;

    fn shr(self, rhs: usize) -> Number {
        let mut shifted = Number::zero();

        for i in rhs..64 {
            shifted.set(i, self.get(i - 1));
        }

        shifted
    }
}


/// Inverts the sign of a Number by flipping bits and adding 1
impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        let mut negated = Number::zero();

        for i in 0..64 {
            negated.set(i, !self.get(i));
        }

        negated + Number::one()
    }
}

/// Basic implementation of full addition circuit
impl Add<Number> for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        let mut rval = Number::zero();
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

/// Simple subtractor that takes two's complement of other and adds to self
impl Sub<Number> for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        self + -other
    }
}

/// Multiplier that uses basic series of shifts and adding partial products
impl Mul<Number> for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        let mut accumulator = Number::zero();

        // For each Bit in the multiplier, starting at least significant...
        for i in (0..64).rev() {
            let multiplier = other.get(i);

            // ... create a zeroed Number to hold the partial product...
            let mut partial = Number::zero();

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

            accumulator = accumulator + partial;
        }

        accumulator
    }
}

/// Emulates long division in a comically long fashion, which is definitely
/// not how computers do it /shrug
impl Div<Number> for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        // Take "absolute value" of the binaries for simpler math,
        // storing whether final quotient should be negative
        let negate = self.is_negative() != other.is_negative();

        let dividend = if self.is_negative() {
            -self
        } else {
            self
        };

        let divisor = if other.is_negative() {
            -other
        } else {
            other
        };

        let mut quotient = Number::zero();

        // The partial dividend starts at "zero", with each successive round
        // shifting the next bit from the original number
        let mut partial_dividend = Number::zero();

        for i in 0..64 {
            // Shift partial left and assign next bit to least significant
            partial_dividend = partial_dividend << 1;
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
                partial_dividend = partial_dividend - divisor;
            }
        }

        if negate {
            -quotient
        } else {
            quotient
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Number, Bit};

    #[test]
    fn test_shl() {
        let one = Number::one();
        let mut two = Number::zero();
        two.set(62, Bit::On);

        let shifted = &one << 1;

        assert_ne!(one, two);
        assert_eq!(shifted, two);
    }

    #[test]
    fn test_shr() {
        let one = Number::one();
        let mut two = Number::zero();
        two.set(62, Bit::On);

        let shifted = &two >> 1;

        assert_eq!(shifted, one);
    }

    #[test]
    fn test_partial_eq() {
        assert_ne!(Number::zero(), Number::one());
    }

    #[test]
    fn from_int_test() {
        let zero = Number::zero();
        let one = Number::one();
        let negative_one = Number::of(Bit::On);

        let min = {
            let mut ba = Number::of(Bit::Off);
            ba.set(0, Bit::On);
            ba
        };

        let max = {
            let mut ba = Number::of(Bit::On);
            ba.set(0, Bit::Off);
            ba
        };

        assert_eq!(Number::from_int(0), zero, "0");
        assert_eq!(Number::from_int(1), one, "1");
        assert_eq!(Number::from_int(-1), negative_one, "-1");
        assert_eq!(Number::from_int(-9223372036854775808), min, "min");
        assert_eq!(Number::from_int(9223372036854775807), max, "max");
    }

    #[test]
    fn to_int_test() {
        let zero = Number::zero();
        let one = Number::one();
        let negative_one = Number::of(Bit::On);

        let min = {
            let mut ba = Number::of(Bit::Off);
            ba.set(0, Bit::On);
            ba
        };

        let max = {
            let mut ba = Number::of(Bit::On);
            ba.set(0, Bit::Off);
            ba
        };

        assert_eq!(Number::to_int(zero), Ok(0), "0");
        assert_eq!(Number::to_int(one), Ok(1), "1");
        assert_eq!(Number::to_int(negative_one), Ok(-1), "-1");
        assert_eq!(Number::to_int(min), Ok(::std::i64::MIN), "min");
        assert_eq!(Number::to_int(max), Ok(::std::i64::MAX), "max");
    }
}
