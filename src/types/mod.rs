pub mod binary;
pub mod bit;

pub use self::binary::Binary;
pub use self::bit::Bit;

pub type Num = i64;
pub type OperationResult = Result<Num, ::std::num::ParseIntError>;
