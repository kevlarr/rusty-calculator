pub type BitArray = [bool; 8];
pub type Circuit = fn (BitArray, BitArray) -> BitArray;
pub type Num = i8;
pub type OperationResult = Result<Num, ::std::num::ParseIntError>;