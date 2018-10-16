mod gates;

use super::types::{Bit, BitArray};

/// Basic, naive implementation of full addition circuit
pub fn binary_adder(xb: BitArray, yb: BitArray) -> BitArray {
    let mut rval = [false; 64];
    let mut carry = false;

    for i in 0..64 {
        let result = match i {
            0 => half_adder(xb[i], yb[i]),
            _ => full_adder(xb[i], yb[i], carry),
        };

        rval[i] = result.0;
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
    let mut accumulator = [false; 64];

    for i in 0..64 {
        let mut partial = [false; 64];

        for j in 0..(64- i) {
            partial[j + i] = multiplier(xb[j], yb[i]);
        }

        accumulator = binary_adder(accumulator, partial);
    }

    accumulator
}

/// Inverts the sign of a number by flipping bits and adding 1
pub fn complement(b: BitArray) -> BitArray {
    let mut flipped = [false; 64];

    for i in 0..64 {
        flipped[i] = !b[i];
    }

    let mut one = [false; 64];
    one[0] = true;

    binary_adder(flipped, one)
}

/// Single-bit multiplier circuit
fn multiplier(b1: Bit, b2: Bit) -> Bit {
    gates::and(b1, b2)
}

/// Single-bit adder circuit for two inputs
fn half_adder(b1: Bit, b2: Bit) -> (Bit, Bit) {
    (gates::xor(b1, b2), gates::and(b1, b2))
}

/// Single-bit adder circuit for three inputs
fn full_adder(b1: Bit, b2: Bit, b3: Bit) -> (Bit, Bit) {
    let (sum1, carry1) = half_adder(b1, b2);
    let (sum2, carry2) = half_adder(sum1, b3);

    (sum2, gates::or(carry1, carry2))
}
