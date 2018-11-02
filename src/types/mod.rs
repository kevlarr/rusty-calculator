pub mod binary;
pub mod bit;

pub use self::binary::Binary;
pub use self::bit::Bit;

pub use ::std::num::ParseIntError;

pub type ConversionResult = Result<i64, ParseIntError>;
pub type Calculation = fn(i64, i64) -> ConversionResult;
