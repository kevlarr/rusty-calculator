/// Using an 8-bit data type to represent a single bit? Yay!
pub type Bit = bool;

/// Array of Bits ordered from least to most significant
pub type BitArray = [Bit; 64];

/// An "electronic circuit" that acts on full binary numbers
pub type Circuit = fn (BitArray, BitArray) -> BitArray;

/// The number type for the calculator
pub type Num = i64;

/// The result of an operation
pub type OperationResult = Result<Num, ::std::num::ParseIntError>;