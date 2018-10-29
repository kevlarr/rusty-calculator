use super::{circuits, types::{Bit, BitArray, Num, OperationResult}};

/// Converts an integer to an array of "bits" ordered from least significant to most
pub fn to_bit_array(x: Num) -> BitArray {
    // Need 66 chars to represent 64bit, since it adds "0b" to beginning
    let bit_string = format!("{:#066b}", x);
    let mut ba = BitArray::zero();

    bit_string.chars().skip(2)
        .map(|c| if c == '1' { Bit::On } else { Bit::Off })
        .enumerate()
        .for_each(|(i, bit)| ba.set(63 - i, bit));

    ba
}

/// Converts an array of "bits" (ordered from least to most significant) to an integer
pub fn from_bit_array(ba: BitArray) -> OperationResult {
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
    let mut to_convert = ba;

    if to_convert.is_negative() {
        // Take complement to make "positive" and then add sign
        s.push('-');
        to_convert = circuits::complement(to_convert);
    }

    // Run through bit array and push chars onto string, reversing order of bits
    for i in 0..64 {
        s.push(if to_convert.is_on(63 - i) { '1' } else { '0' });
    }

    Num::from_str_radix(s.as_str(), 2)
}

#[cfg(test)]
mod tests {
    use super::{Bit, BitArray};

    #[test]
    fn to_bit_array_test() {
        let zero = BitArray::zero();
        let one = BitArray::one();
        let negative_one = BitArray::of(Bit::On);

        let min = {
            let mut ba = BitArray::of(Bit::Off);
            ba.set(63, Bit::On);
            ba
        };

        let max = {
            let mut ba = BitArray::of(Bit::On);
            ba.set(63, Bit::Off);
            ba
        };

        assert_eq!(super::to_bit_array(0), zero, "0");
        assert_eq!(super::to_bit_array(1), one, "1");
        assert_eq!(super::to_bit_array(-1), negative_one, "-1");
        assert_eq!(super::to_bit_array(-9223372036854775808), min, "min");
        assert_eq!(super::to_bit_array(9223372036854775807), max, "max");

    }

    #[test]
    fn from_bit_array_test() {

    }
}
