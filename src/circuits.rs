use super::types::{Bit, BitArray};

/// Basic, naive implementation of full addition circuit
pub fn binary_adder(xb: BitArray, yb: BitArray) -> BitArray {
    let mut rval = BitArray::zero();
    let mut carry = Bit::Off;

    for i in 0..64 {
        let result = match i {
            0 => half_adder(xb.get(i), yb.get(i)),
            _ => full_adder(xb.get(i), yb.get(i), carry),
        };

        rval.set(i, result.0);
        carry = result.1;
    }

    rval
}

/// Subtractor that takes complement of yb and adds to xb
pub fn binary_subtractor(xb: BitArray, yb: BitArray) -> BitArray {
    binary_adder(xb, complement(yb))
}

/// Multiplier that uses naive, unoptimized series of shifts and adding partial products
pub fn binary_multiplier(xb: BitArray, yb: BitArray) -> BitArray {
    let mut accumulator = BitArray::zero();

    for i in 0..64 {
        let mut partial = BitArray::zero();

        for j in 0..(64- i) {
            partial.set(j + i, multiplier(xb.get(j), yb.get(i)));
        }

        accumulator = binary_adder(accumulator, partial);
    }

    accumulator
}

/// Inverts the sign of a number by flipping bits and adding 1
pub fn complement(b: BitArray) -> BitArray {
    let mut flipped = BitArray::zero();

    for i in 0..64 {
        flipped.set(i, !b.get(i));
    }

    binary_adder(flipped, BitArray::one())
}

/// Single-bit adder circuit for two inputs
fn half_adder(b1: Bit, b2: Bit) -> (Bit, Bit) {
    (b1 ^ b2, b1 & b2)
}

/// Single-bit adder circuit for three inputs
fn full_adder(b1: Bit, b2: Bit, b3: Bit) -> (Bit, Bit) {
    let (sum1, carry1) = half_adder(b1, b2);
    let (sum2, carry2) = half_adder(sum1, b3);

    (sum2, carry1 | carry2)
}

/// Single-bit multiplier circuit
fn multiplier(b1: Bit, b2: Bit) -> Bit {
    b1 & b2
}

#[cfg(test)]
mod tests {
    use self::Bit::{Off, On};
    use super::*;

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(Off, Off), (Off, Off));
        assert_eq!(half_adder(On, Off), (On, Off));
        assert_eq!(half_adder(Off, On), (On, Off));
        assert_eq!(half_adder(On, On), (Off, On));
    }

    #[test]
    fn test_full_adder() {
        // No sum, no carry
        assert_eq!(full_adder(Off, Off, Off), (Off, Off));

        // Sum, no carry
        assert_eq!(full_adder(On, Off, Off), (On, Off));
        assert_eq!(full_adder(Off, On, Off), (On, Off));
        assert_eq!(full_adder(Off, Off, On), (On, Off));

        // Carry, no sum
        assert_eq!(full_adder(On, On, Off), (Off, On));
        assert_eq!(full_adder(Off, On, On), (Off, On));
        assert_eq!(full_adder(On, Off, On), (Off, On));

        // Sum, carry
        assert_eq!(full_adder(On, On, On), (On, On));
    }

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier(Off, Off), Off);
        assert_eq!(multiplier(Off, On), Off);
        assert_eq!(multiplier(On, Off), Off);
        assert_eq!(multiplier(On, On), On);
    }
}
