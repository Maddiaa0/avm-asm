use lalrpop_util::*;

use crate::opcodes::Opcode;

#[derive(Debug)]
pub enum Statement {
    OpcodeStatement(Opcode, /*indirect=*/ bool, Vec<u64>), // Opcode and it's operands
    Label(String),
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
