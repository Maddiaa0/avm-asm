use lalrpop_util::*;

pub mod types;

use crate::{opcodes::Opcode, utils::hex_to_bytes};

#[derive(Debug, Clone)]
pub enum Statement {
    IncludeStatement(String),
    MacroStatement(String, Vec<Statement>),
    MacroInvocation(String),
    OpcodeStatement(
        Opcode,
        /*indirect=*/ bool,
        Vec<Operand>,
        /*Label*/ Option<String>,
    ), // Opcode and it's operands
    ConstantDefinition(String, Operand),
    Label(String),
}

#[derive(Debug, Clone)]
pub enum Operand {
    Decimal(u64),
    Hex(String),
    // TOOD: change name of above to fit that this is now just generic operands not literals
    Tag(TypeTag),
    Variable(String),
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
            _ => panic!("A variable should never appear in the final code!"),
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

                hex_to_bytes(hex_str, number_of_bits)
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

// TODO(md): the parser should not be concerned with the file manager, move this up a level
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

// #[test]
// fn test_includes_io() {
//     // Test includes IO
//     let input = std::fs::read_to_string("./test_programs/includes.avm").unwrap();
//     let parsed = parse_asm(input);

//     // Each file contains just one macro, so we expect that they end up pointing at the same thing
//     let expected_length = 2;
//     assert_eq!(parsed.len(), expected_length);
// }

// Next test: make labels work in the multi file setting
