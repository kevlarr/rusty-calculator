pub fn add(x: i32, y: i32) -> i32 {
    let mut rval = String::with_capacity(32);
    let xb = to_bit_array(x);
    let yb = to_bit_array(y);

    // Iterate through each pair of matching bits and send through adders
    let mut carry = false;

    for i in 0..32 {
        let result = match i {
            0 => half_adder(xb[i], yb[i]),
            _ => full_adder(xb[i], yb[i], carry),
        };

        rval.push(if result.0 { '1' } else { '0' });
        carry = result.1;
    }

    i32::from_str_radix(
        rval.chars().rev().collect::<String>().as_str(),
        2,
    ).unwrap()
}

fn half_adder(b1: bool, b2: bool) -> (bool, bool) {
    (b1 ^ b2, b1 && b2)
}

fn full_adder(b1: bool, b2: bool, b3: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(b1, b2);
    let (sum2, carry2) = half_adder(sum1, b3);

    (sum2, carry1 || carry2)
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

    // FIXME use iterators instead of creating arrays
    bit_string.chars().skip(2)
        .map(|c| c == '1')
        .enumerate()
        .for_each(|(i, bit)| arr[31 - i] = bit);

    arr
}
