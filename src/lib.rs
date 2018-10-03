pub mod operations {
    pub fn add(x: i32, y: i32) -> Result<i32, ::std::num::ParseIntError> {
        let mut rval = String::with_capacity(32);
        let xb = super::to_bit_array(x);
        let yb = super::to_bit_array(y);

        // Iterate through each pair of matching bits and send through adders
        let mut carry = false;

        for i in 0..32 {
            let result = match i {
                0 => super::circuits::half_adder(xb[i], yb[i]),
                _ => super::circuits::full_adder(xb[i], yb[i], carry),
            };

            rval.push(if result.0 { '1' } else { '0' });
            carry = result.1;
        }

        i32::from_str_radix(
            rval.chars().rev().collect::<String>().as_str(),
            2,
        )
    }

    pub fn subtract(x: i32, y: i32) -> Result<i32, ::std::num::ParseIntError> {
        // FIXME
        Ok(x - y)
    }

    pub fn multiply(x: i32, y: i32) -> Result<i32, ::std::num::ParseIntError> {
        // FIXME
        Ok(x * y)
    }

    pub fn divide(x: i32, y: i32) -> Result<i32, ::std::num::ParseIntError> {
        // FIXME
        Ok(x / y)
    }
}

mod gates {
    pub fn and(b1: bool, b2: bool) -> bool { b1 && b2 }

    pub fn or(b1: bool, b2: bool) -> bool { b1 || b2 }

    pub fn xor(b1: bool, b2: bool) -> bool { b1 ^ b2 }
}

mod circuits {
    pub fn half_adder(b1: bool, b2: bool) -> (bool, bool) {
        (super::gates::xor(b1, b2), super::gates::and(b1, b2))
    }

    pub fn full_adder(b1: bool, b2: bool, b3: bool) -> (bool, bool) {
        let (sum1, carry1) = half_adder(b1, b2);
        let (sum2, carry2) = half_adder(sum1, b3);

        (sum2, super::gates::or(carry1, carry2))
    }
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
