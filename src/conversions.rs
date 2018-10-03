/// Converts an integer to an array of "bits" ordered from least significant to most
pub fn to_bit_array(x: i32) -> [bool; 32] {
    // Need 34 chars to represent 32bit, since it adds "0b" to beginning
    let bit_string = format!("{:#034b}", x);
    let mut arr = [false; 32];

    // FIXME use iterators instead of creating arrays
    bit_string.chars().skip(2)
        .map(|c| c == '1')
        .enumerate()
        .for_each(|(i, bit)| arr[31 - i] = bit);

    arr
}
