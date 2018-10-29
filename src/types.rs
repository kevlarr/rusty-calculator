use ::std::ops::{BitAnd, BitOr, BitXor, Not};
use ::std::fmt;
use ::std::cmp::PartialEq;


#[derive(Copy, Clone, PartialEq)]
pub enum Bit { Off, On }

use self::Bit::*;

impl fmt::Debug for Bit {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Off => 0.fmt(formatter),
			On  => 1.fmt(formatter),
		}
	}
}


impl BitAnd for Bit {
	type Output = Self;

	fn bitand(self, other: Self) -> Self {
		match (self, other) {
			(On, On) => On,
			_ => Off,
		}
	}
}

impl BitOr for Bit {
	type Output = Self;

	fn bitor(self, other: Self) -> Self {
		match (self, other) {
			(On, _) | (_, On) => On,
			_ => Off,
		}
	}
}

impl BitXor for Bit {
	type Output = Self;

	fn bitxor(self, other: Self) -> Self {
		if self == other { Off } else { On }
	}
}

impl Not for Bit {
	type Output = Self;

	fn not(self) -> Self {
		match self {
			Off => On,
			On  => Off,
		}
	}
}

/// Array of Bits ordered from least to most significant
pub struct BitArray([Bit; 64]);

impl BitArray {
	pub fn of(b: Bit) -> BitArray {
		BitArray([b; 64])
	}

	pub fn zero() -> BitArray {
		BitArray::of(Off)
	}

	pub fn one() -> BitArray {
		let mut ba = BitArray::zero();
		ba.set(0, On);
		ba
	}

	pub fn get(&self, i: usize) -> Bit {
		self.0[i]
	}

	pub fn set(&mut self, i: usize, b: Bit) {
		self.0[i] = b;
	}

	pub fn is_on(&self, i: usize) -> bool {
		match self.0[i] {
			On => true,
			_ => false,
		}
	}

	pub fn is_negative(&self) -> bool {
		self.is_on(63)
	}
}

impl fmt::Debug for BitArray {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0[..].fmt(formatter)
    }
}

impl PartialEq for BitArray {
	fn eq(&self, other: &BitArray) -> bool {
		for i in 0..63 {
			if self.get(i) != other.get(i) {
				return false;
			}
		}

		return true;
	}
}


pub type Circuit = fn (BitArray, BitArray) -> BitArray;
pub type Num = i64;
pub type OperationResult = Result<Num, ::std::num::ParseIntError>;