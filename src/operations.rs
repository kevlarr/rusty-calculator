use super::{circuits, conversions};

pub fn add(x: i8, y: i8) -> Result<i8, ::std::num::ParseIntError> {
    let xb = conversions::to_bit_array(x);
    let yb = conversions::to_bit_array(y);

    println!("x: {}\n{:#010b}\n{:?}", x, x, xb);
    println!("y: {}\n{:#010b}\n{:?}", y, y, yb);

    conversions::from_bit_array(
        circuits::binary_adder(xb, yb))
}

pub fn subtract(x: i8, y: i8) -> Result<i8, ::std::num::ParseIntError> {
    add(x, -y)
}

pub fn multiply(x: i8, y: i8) -> Result<i8, ::std::num::ParseIntError> {
    // FIXME
    Ok(x * y)
}

pub fn divide(x: i8, y: i8) -> Result<i8, ::std::num::ParseIntError> {
    // FIXME
    Ok(x / y)
}


#[cfg(test)]
mod tests {
    use super::{add};

    #[test]
    fn add_works_for_positives() {
        let args = [
            (0, 1), (0, 100), (0, 500),
            (1, 2), (1, 200), (1, 200000),
            (123321, 192392), (98498239, 1238723),
            (0, ::std::i8::MAX)
        ];
        for (x, y) in args.iter() {
            assert_eq!(add(*x, *y), Ok(x + y));
        }
    }

    #[test]
    fn add_works_for_negatives() {
        let args = [
            (0, -1), (0, -100), (0, -500),
            (1, -2), (1, -200), (1, -200000),
            (-1, -2), (-1, -200), (-1, -200000),
            (-123321, -192392), (-98498239, 1238723),
            (0, ::std::i8::MIN), (1, ::std::i8::MIN),
        ];
        for (x, y) in args.iter() {
            assert_eq!(add(*x, *y), Ok(x + y), "\nTesting: {} + {}", x, y);
        }
    }
}
