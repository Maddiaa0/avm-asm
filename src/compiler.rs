// Compiler
// Read in the AST from the parser

use crate::{
    codegen::generate_code,
    instruction::Instruction,
    parser::{parse_asm, Statement},
};

pub fn compile_file(path: &String) -> String {
    let file = std::fs::read_to_string(path).unwrap();
    compile_asm(file)
}

pub fn compile_asm(input: String) -> String {
    let parsed = parse_asm(input);

    // Before we pass to the code generator, all we should have is a vector of opcodes
    let instructions = temporary_to_instruction_vector(parsed);
    generate_code(instructions)
}

// This will be replaced with methods that resolve
// 1. labels
// 2. macros
fn temporary_to_instruction_vector(parsed: Vec<Statement>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for statement in parsed {
        if let Statement::OpcodeStatement(opcode, indirect, operands) = statement {
            let instr = Instruction::new(opcode, indirect, operands);
            instructions.push(instr);
        }
    }

    instructions
}

#[test]
fn simple_test_compile() {
    let input = "
        add 1 2 3;
        sub! 1 2 3;
    "
    .to_owned();

    let bytecode = compile_asm(input);

    assert_eq!(bytecode, "00000000000000000001000000000000000200000000000000030101000000000000000100000000000000020000000000000003");
}
