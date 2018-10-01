pub fn add(x: i32, y: i32) -> i32 {
    let xb = to_bit_array(x);
    let yb = to_bit_array(y);

    x + y
}

pub fn subtract(x: i32, y: i32) -> i32 {
    // FIXME
    x - y
}

pub fn multiply(x: i32, y: i32) -> i32 {
    // FIXME
    x * y
}

pub fn divide(x: i32, y: i32) -> i32 {
    // FIXME
    x / y
}

/// Converts an integer to an array of "bits" ordered from least significant to most
fn to_bit_array(x: i32) -> [bool; 32] {
    // Need 34 chars to represent 32bit, since it adds "0b" to beginning
    let bit_string = format!("{:#034b}", x);
    let mut arr = [false; 32];

    bit_string.chars().skip(2)
        .map(|c| c == '1')
        .enumerate()
        // bit string is ordered from most significant to least,
        // so adding to array needs to happen in reverse order
        .for_each(|(i, bit)| arr[31 - i] = bit);

    arr
}
