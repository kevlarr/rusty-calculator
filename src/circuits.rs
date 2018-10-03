use super::gates;

pub fn half_adder(b1: bool, b2: bool) -> (bool, bool) {
    (gates::xor(b1, b2), gates::and(b1, b2))
}

pub fn full_adder(b1: bool, b2: bool, b3: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(b1, b2);
    let (sum2, carry2) = half_adder(sum1, b3);

    (sum2, gates::or(carry1, carry2))
}
