// Compiler
// Read in the AST from the parser

use std::collections::{HashMap, VecDeque};

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

    let mut parsed = resolve_macros(parsed);

    // Resolve all static labels
    resolve_labels(&mut parsed);

    // Before we pass to the code generator, all we should have is a vector of opcodes
    let instructions = temporary_to_instruction_vector(parsed);
    generate_code(instructions)
}

// Resolve macros
//
// This algorithm involves two passes:
// 1. collect all macro definitions into a hash map
// 2. resolve all macro invocations
fn resolve_macros(parsed: Vec<Statement>) -> Vec<Statement> {
    let macro_definitions = collect_macro_definitions(&parsed);
    expand_macros(parsed, &macro_definitions)
}

fn collect_macro_definitions(parsed: &Vec<Statement>) -> HashMap<String, Vec<Statement>> {
    let mut macro_definitions: HashMap<String, Vec<Statement>> = HashMap::new();

    for statement in parsed.iter() {
        match statement {
            Statement::MacroStatement(name, statements) => {
                macro_definitions.insert(name.clone(), statements.clone());
            }
            _ => {}
        }
    }

    macro_definitions
}

// Expand Macros
//
// Expand macros using a stack based approach, to handle nested macro definitions
fn expand_macros(
    parsed: Vec<Statement>,
    macro_definitions: &HashMap<String, Vec<Statement>>,
) -> Vec<Statement> {
    let mut resolved: Vec<Statement> = Vec::new();
    let mut stack = VecDeque::new();

    // Push ast nodes onto stack in reverse, without macro defs
    for node in parsed
        .iter()
        .filter(|node| !matches!(node, Statement::MacroStatement(_, _)))
        .rev()
    {
        stack.push_back(node.clone());
    }

    while let Some(node) = stack.pop_back() {
        match node {
            Statement::MacroInvocation(name) => {
                // TODO: dont panic if macro is undefined
                let macro_def = macro_definitions.get(&name).unwrap();
                for statement in macro_def.iter().rev() {
                    stack.push_back(statement.clone());
                }
            }
            _ => resolved.push(node),
        }
    }

    resolved
}

// Resolve labels
//
// This algorithm involves two passes:
// 1. Collect all of the labels
// 2. Resolve the labels in place
fn resolve_labels(parsed: &mut [Statement]) {
    let mut label_map: HashMap<String, u64> = HashMap::new();

    // First pass - label collection
    let mut pc = 0;
    for ref statement in parsed.iter_mut() {
        match statement {
            Statement::Label(label) => {
                label_map.insert(label.clone(), pc);
            }
            Statement::OpcodeStatement(_, _, _, _) => {
                pc += 1;
            }
            _ => panic!("Only opcodes and labels should remain"),
        }
    }

    // Second pass - label resolution
    for statement in parsed.iter_mut() {
        match statement {
            Statement::Label(_) => {}
            Statement::OpcodeStatement(_, _, operands, label) => {
                // TODO: error handling for unmatched label
                // TODO: make sure the code with the label IS a JUMP
                if let Some(ref label) = label {
                    let resolved_label = label_map.get(label).unwrap();
                    // If it is a jump we should be able to assume that the operands are empty
                    operands.push(*resolved_label);
                }
            }
            _ => panic!("Only opcodes and labels should remain"),
        }
    }
}

// This will be replaced with methods that resolve
// 1. labels
// 2. macros
fn temporary_to_instruction_vector(parsed: Vec<Statement>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for statement in parsed {
        if let Statement::OpcodeStatement(opcode, indirect, operands, _) = statement {
            // At this point labels should have been resolved!
            let instr = Instruction::new(opcode, indirect, operands);
            instructions.push(instr);
        }
    }

    instructions
}

mod tests {
    use crate::{
        codegen::generate_code, compiler::compile_asm, instruction::Instruction, opcodes::Opcode,
    };

    #[test]
    fn simple_test() {
        let input = "
            add 1 2 3;
            sub! 1 2 3;
        "
        .to_owned();

        let bytecode = compile_asm(input);

        assert_eq!(bytecode, "00000000000000000001000000000000000200000000000000030101000000000000000100000000000000020000000000000003");
    }

    #[test]
    fn simple_label_test() {
        let input = "
            add 1 2 3;
            jump @label;
            sub 1 2 3;
        label:
            add 1 2 3;

        "
        .to_owned();

        let expected_instructions = vec![
            Instruction::new(Opcode::ADD, false, vec![1, 2, 3]),
            Instruction::new(Opcode::JUMP, false, vec![3]),
            Instruction::new(Opcode::SUB, false, vec![1, 2, 3]),
            Instruction::new(Opcode::ADD, false, vec![1, 2, 3]),
        ];
        let expected_bytecode = generate_code(expected_instructions);

        let bytecode = compile_asm(input);
        assert_eq!(bytecode, expected_bytecode);
    }

    #[test]
    fn simple_macro() {
        let input = "
            .macro test {
                add 1 2 3;
                sub 1 2 3;
            };

            $test;
        "
        .to_owned();

        let expected_instructions = vec![
            Instruction::new(Opcode::ADD, false, vec![1, 2, 3]),
            Instruction::new(Opcode::SUB, false, vec![1, 2, 3]),
        ];
        let expected_bytecode = generate_code(expected_instructions);

        let bytecode = compile_asm(input);
        assert_eq!(bytecode, expected_bytecode);
    }

    #[test]
    fn nested_macros() {
        let input = "
            .macro test {
                add 1 2 3;
                sub 1 2 3;
            };

            .macro test2 {
                $test;
                add 1 2 3;
            };

            $test2;
        "
        .to_owned();

        let expected_instructions = vec![
            Instruction::new(Opcode::ADD, false, vec![1, 2, 3]),
            Instruction::new(Opcode::SUB, false, vec![1, 2, 3]),
            Instruction::new(Opcode::ADD, false, vec![1, 2, 3]),
        ];
        let expected_bytecode = generate_code(expected_instructions);

        let bytecode = compile_asm(input);
        assert_eq!(bytecode, expected_bytecode);
    }
}
