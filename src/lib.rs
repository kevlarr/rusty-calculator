pub mod types;

use types::{Binary, Num, OperationResult};

pub fn add(x: Num, y: Num) -> OperationResult {
    (Binary::from_int(x) + Binary::from_int(y)).to_int()
}

pub fn subtract(x: Num, y: Num) -> OperationResult {
    (Binary::from_int(x) - Binary::from_int(y)).to_int()
}

pub fn multiply(x: Num, y: Num) -> OperationResult {
    (Binary::from_int(x) * Binary::from_int(y)).to_int()
}

pub fn divide(x: Num, y: Num) ->  OperationResult {
    (Binary::from_int(x) / Binary::from_int(y)).to_int()
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_test() {
        let args = [
            (0, 0), (0, 1), (0, 100), (0, 500),
            (1, 2), (1, 200), (1, 200000),
            (123321, 192392), (98498239, 1238723),
            (0, ::std::i64::MAX),

            (-1, 0), (0, -1), (0, -100), (0, -500),
            (1, -2), (1, -200), (1, -200000),
            (-1, -2), (-1, -200), (-1, -200000),
            (-123321, -192392), (-98498239, 1238723),
            (0, ::std::i64::MIN), (1, ::std::i64::MIN),

            (123, -32), (583, -91283), (-912389, 49823),
            (::std::i64::MIN, ::std::i64::MAX)
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::add(*x, *y), Ok(x + y), "{} + {}", x, y);
        }
    }

    #[test]
    fn subtract_test() {
        let args = [
            (0, 1), (0, 100), (0, 500),
            (1, 2), (1, 200), (1, 200000),
            (123321, 192392), (98498239, 1238723),
            (1, ::std::i64::MAX),

            (0, -1), (0, -100), (0, -500),
            (1, -2), (1, -200), (1, -200000),
            (-1, -2), (-1, -200), (-1, -200000),
            (-123321, -192392), (-98498239, 1238723),
            (-1, ::std::i64::MIN), (2, ::std::i64::MAX),

            (123, -32), (583, -91283), (-912389, 49823)
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::subtract(*x, *y), Ok(x - y), "{} - {}", x, y);
        }
    }

    #[test]
    fn multiply_test() {
        let args = [
            (0, 0), (0, 1), (1, 0),
            (0, -1), (-1, 0), (1, -1),
            (1023, 983298), (9183032, 9283983),
            (-51923, -938983), (-98329838, -29389238),
            (29389, -9238982), (-9238983, 928392838)
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::multiply(*x, *y), Ok(x * y), "{} * {}", x, y);
        }
    }
}
