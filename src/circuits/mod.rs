mod gates;

use super::types::{BitArray};

/// Basic circuit for adding "8-bit" arrays ordered least to most significant
pub fn binary_adder(xb: BitArray, yb: BitArray) -> BitArray {
    let mut rval = [false; 8];
    let mut carry = false;

    for i in 0..8 {
        let result = match i {
            0 => half_adder(xb[i], yb[i]),
            _ => full_adder(xb[i], yb[i], carry),
        };

        rval[i] = result.0;
        carry = result.1;
    }

    rval
}

/// Single-bit adder circuit for two inputs
fn half_adder(b1: bool, b2: bool) -> (bool, bool) {
    (gates::xor(b1, b2), gates::and(b1, b2))
}

/// Single-bit adder circuit for three inputs
fn full_adder(b1: bool, b2: bool, b3: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(b1, b2);
    let (sum2, carry2) = half_adder(sum1, b3);

    (sum2, gates::or(carry1, carry2))
}
