// Compiler
// Read in the AST from the parser

use crate::{codegen::generate_code, instruction::Instruction, parser::{parse_asm, Statement}};

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
        match statement {
            Statement::OpcodeStatement(opcode, operands) => {
                let instr = Instruction::new(opcode, operands);
                instructions.push(instr);
            }
            // Right now as we are super simple, we do not need to handle any other labels
            _ => {}
        }
    }

    instructions
}

#[test]
fn simple_test_compile() {
    let input = "
        add 1 2 3;
        sub 1 2 3;
    ".to_owned();

    let bytecode = compile_asm(input);

    assert_eq!(bytecode, "0000000000000000010000000000000002000000000000000301000000000000000100000000000000020000000000000003");
}
