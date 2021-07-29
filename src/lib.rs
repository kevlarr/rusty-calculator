macro_rules! map {
    ( $( $key:expr => $val:expr),* ) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};

    ( $( $key:expr => $val:expr,)* ) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};
}

mod binary;
mod bit;

pub mod lexer;
pub mod parser;

pub use binary::Binary;
pub use bit::Bit;

pub type ParseResult = Result<i64, ::std::num::ParseIntError>;

#[cfg(test)]
mod tests {
    #[test]
    fn add_test() {
        let args = [
            (0, 0),
            (0, 1),
            (0, 100),
            (0, 500),
            (1, 2),
            (1, 200),
            (1, 200000),
            (123321, 192392),
            (98498239, 1238723),
            (0, ::std::i64::MAX),
            (-1, 0),
            (0, -1),
            (0, -100),
            (0, -500),
            (1, -2),
            (1, -200),
            (1, -200000),
            (-1, -2),
            (-1, -200),
            (-1, -200000),
            (-123321, -192392),
            (-98498239, 1238723),
            (0, ::std::i64::MIN),
            (1, ::std::i64::MIN),
            (123, -32),
            (583, -91283),
            (-912389, 49823),
            (::std::i64::MIN, ::std::i64::MAX),
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::add(*x, *y), Ok(x + y), "{} + {}", x, y);
        }
    }

    #[test]
    fn subtract_test() {
        let args = [
            (0, 1),
            (0, 100),
            (0, 500),
            (1, 2),
            (1, 200),
            (1, 200000),
            (123321, 192392),
            (98498239, 1238723),
            (1, ::std::i64::MAX),
            (0, -1),
            (0, -100),
            (0, -500),
            (1, -2),
            (1, -200),
            (1, -200000),
            (-1, -2),
            (-1, -200),
            (-1, -200000),
            (-123321, -192392),
            (-98498239, 1238723),
            (-1, ::std::i64::MIN),
            (2, ::std::i64::MAX),
            (123, -32),
            (583, -91283),
            (-912389, 49823),
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::subtract(*x, *y), Ok(x - y), "{} - {}", x, y);
        }
    }

    #[test]
    fn multiply_test() {
        let args = [
            (0, 0),
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, -1),
            (3, 5),
            (1023, 983298),
            (9183032, 9283983),
            (-51923, -938983),
            (-98329838, -29389238),
            (29389, -9238982),
            (-9238983, 928392838),
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::multiply(*x, *y), Ok(x * y), "{} * {}", x, y);
        }
    }

    #[test]
    fn divide_test() {
        let args = [
            (10, 4),
            (4, 2),
            (123, 13),
            (13, 123),
            (0, 1),
            (0, 1),
            (1, 1),
            (0, -1),
            (-1, 1),
            (1, -1),
            (3, 5),
            (1023, 983298),
            (9183032, 9283983),
            (-51923, -938983),
            (-98329838, -29389238),
            (29389, -9238982),
            (-9238983, 928392838),
        ];

        for (x, y) in args.iter() {
            assert_eq!(super::divide(*x, *y), Ok(x / y), "{} * {}", x, y);
        }
    }
}
