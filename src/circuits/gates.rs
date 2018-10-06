use super::{Bit};

pub fn and(b1: Bit, b2: Bit) -> Bit { b1 && b2 }

pub fn or(b1: Bit, b2: Bit) -> Bit { b1 || b2 }

pub fn xor(b1: Bit, b2: Bit) -> Bit { b1 ^ b2 }
