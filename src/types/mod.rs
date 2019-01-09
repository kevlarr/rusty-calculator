pub mod binary;
pub mod bit;
pub mod expression;

pub use self::binary::Binary;
pub use self::bit::Bit;
pub use self::expression::Expression;

pub type Operation = fn(i64, i64) -> ParseResult;
pub type ParseResult = Result<i64, ::std::num::ParseIntError>;
