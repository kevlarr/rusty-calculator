use super::{circuits, conversions};

pub fn add(x: i32, y: i32) -> Result<i32, ::std::num::ParseIntError> {
    let mut rval = String::with_capacity(32);
    let xb = conversions::to_bit_array(x);
    let yb = conversions::to_bit_array(y);

    // Iterate through each pair of matching bits and send through adders
    let mut carry = false;

    for i in 0..32 {
        let result = match i {
            0 => circuits::half_adder(xb[i], yb[i]),
            _ => circuits::full_adder(xb[i], yb[i], carry),
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
