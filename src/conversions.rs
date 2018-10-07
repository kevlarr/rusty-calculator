use super::{circuits, types::{BitArray, Num, OperationResult}};

/// Converts an integer to an array of "bits" ordered from least significant to most
pub fn to_bit_array(x: Num) -> BitArray {
    // Need 10 chars to represent 8bit, since it adds "0b" to beginning
    let bit_string = format!("{:#010b}", x);
    let mut arr = [false; 8];

    bit_string.chars().skip(2)
        .map(|c| c == '1')
        .enumerate()
        .for_each(|(i, bit)| arr[7 - i] = bit);

    arr
}

/// Converts an array of "bits" (ordered from least to most significant) to an integer
pub fn from_bit_array(arr: BitArray) -> OperationResult {
    // Rust is a little inconsistent with how it handles negative binary numbers...
    //
    //   - Literals use negative, eg. -0b0000_0011 for -3)
    //   - format! outputs two's complement, eg. "0b1111_1101" for `format!("{:#010b}", -3)`
    //   - i8::from_str_radix expects a stringified version of a negative literal, rather than
    //      two's complement
    //
    // ... which means we can't simply format! -> build complement string w/ logic -> parse with
    // from_str_radix. If negative, need to convert from two's complement to negative literal
    let mut s = String::with_capacity(9);
    let mut arr_to_convert = arr;

    if arr[7] {
        s.push('-');

        // Is negative, so subtract one, flip bits, and add sign to str before parsing
        arr_to_convert = circuits::binary_adder(arr, [true; 8]);

        for i in 0..8 {
            arr_to_convert[i] = !arr_to_convert[i];
        }
    }

    // Run through bit array and push chars onto string, reversing order of bits
    for i in 0..8 {
        s.push(if arr_to_convert[7 - i] { '1' } else { '0' });
    }

    println!("s: {}", s);

    Num::from_str_radix(s.as_str(), 2)
}
