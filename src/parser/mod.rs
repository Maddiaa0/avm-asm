use lalrpop_util::*;

use crate::{opcodes::Opcode, utils::hex_to_bytes};

#[derive(Debug, Clone)]
pub enum Statement {
    MacroStatement(String, Vec<Statement>),
    MacroInvocation(String),
    OpcodeStatement(
        Opcode,
        /*indirect=*/ bool,
        Vec<Operand>,
        Option<String>,
    ), // Opcode and it's operands
    Label(String),
}

#[derive(Debug, Clone)]
pub enum Operand {
    Decimal(u64),
    Hex(String),
    // TOOD: change name of above to fit that this is now just generic operands not literals
    Tag(TypeTag),
}

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

impl Operand {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        match self {
            Operand::Decimal(value) => value.to_be_bytes().to_vec(),
            Operand::Hex(value) => {
                let value = u64::from_str_radix(&value[2..], 16).unwrap();
                value.to_be_bytes().to_vec()
            }
            Operand::Tag(tag) => (tag.clone() as u8).to_be_bytes().to_vec(),
        }
    }

    pub fn to_be_bytes_with_hint(&self, tag_hint: TypeTag) -> Vec<u8> {
        match self {
            Operand::Hex(hex_str) => {
                let number_of_bits = match tag_hint {
                    TypeTag::U8 => 8,
                    TypeTag::U16 => 16,
                    TypeTag::U32 => 32,
                    TypeTag::U64 => 64,
                    TypeTag::U128 => 128,
                    TypeTag::FF => 256,
                };

                hex_to_bytes(&hex_str, number_of_bits)
            }
            _ => self.to_be_bytes(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeTag {
    U8,
    U16,
    U32,
    U64,
    U128,
    FF,
}

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

pub(crate) fn parse_asm(input: String) -> Vec<Statement> {
    let parser = avm::StatementsParser::new();
    let parsed = parser.parse(&input);
    parsed.unwrap()
}

lalrpop_mod!(avm);

#[test]
fn test_parser() {
    let input = "
        add 1 2 3;
    some_label:
        sub 1 2 3;
    "
    .to_owned();

    parse_asm(input);
}
