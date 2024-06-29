use super::{Operand, TypeTag};

// Opcode trait implementations
impl From<u8> for Operand {
    fn from(value: u8) -> Self {
        Operand::Decimal(value as u64)
    }
}

impl From<u16> for Operand {
    fn from(value: u16) -> Self {
        Operand::Decimal(value as u64)
    }
}

impl From<u32> for Operand {
    fn from(value: u32) -> Self {
        Operand::Decimal(value as u64)
    }
}

impl From<u64> for Operand {
    fn from(value: u64) -> Self {
        Operand::Decimal(value)
    }
}

impl From<i8> for Operand {
    fn from(value: i8) -> Self {
        Operand::Decimal(value as u64)
    }
}

impl From<i16> for Operand {
    fn from(value: i16) -> Self {
        Operand::Decimal(value as u64)
    }
}

impl From<i32> for Operand {
    fn from(value: i32) -> Self {
        Operand::Decimal(value as u64)
    }
}

impl From<i64> for Operand {
    fn from(value: i64) -> Self {
        Operand::Decimal(value as u64)
    }
}

impl From<String> for Operand {
    fn from(value: String) -> Self {
        if value.starts_with("0x") {
            Operand::Hex(value)
        } else {
            Operand::Decimal(value.parse().unwrap())
        }
    }
}

// Type tag
impl From<u8> for TypeTag {
    fn from(value: u8) -> Self {
        match value {
            0 => TypeTag::U8,
            1 => TypeTag::U16,
            2 => TypeTag::U32,
            3 => TypeTag::U64,
            4 => TypeTag::U128,
            5 => TypeTag::FF,
            _ => panic!("Invalid type tag"),
        }
    }
}
