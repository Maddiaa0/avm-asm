// Compiler
// Read in the AST from the parser

use std::collections::{HashMap, VecDeque};

use crate::{
    codegen::generate_code, fm::FileManager, instruction::Instruction, opcodes::Opcode, parser::{parse_asm, Operand, Statement}
};

pub fn compile_file(path: &String) -> String {
    let file = std::fs::read_to_string(path).unwrap();
    
    // TODO: make the file manager keep track of the parent when dealing with include statements
    // Maybe keep them in a tuple? Or deal with them when extending
    let mut fm = FileManager::new();

    let mut parsed = parse_asm(file);
    fm.extend_file_stack(&parsed);

    while !fm.is_empty() {
        let next_file_contents = fm.get_next_file_contents();
        let mut new_parsed = parse_asm(next_file_contents);

        fm.extend_file_stack(&new_parsed);
        new_parsed = new_parsed.into_iter().filter(|statement| !matches!(statement, Statement::IncludeStatement(_))).collect();

        // Extend the AST with new file contents
        parsed.extend(new_parsed);
    }

    process_asm(parsed)
}

pub fn compile_asm(input: String) -> String {
    let parsed = parse_asm(input);

    process_asm(parsed)
}

pub fn process_asm(mut parsed: Vec<Statement>) -> String {

    // Resolve all constants
    resolve_constants(&mut parsed);

    // TODO: remove to_vec
    let mut parsed = resolve_macros(parsed.to_vec());

    // Resolve all static labels
    resolve_labels(&mut parsed);

    // Before we pass to the code generator, all we should have is a vector of opcodes
    let instructions = temporary_to_instruction_vector(parsed);
    generate_code(instructions)

}

// Resolve constants
//
// This algorithm involves two passes:
// 1. collect all constant definitions into a hash map
// 2. Find all invocations of constants and replace them with the value
fn resolve_constants(parsed: &mut [Statement]) {
    let mut constants: HashMap<String, Operand> = HashMap::new();

    for statement in parsed.iter() {
        if let Statement::ConstantDefinition(name, value) = statement {
            constants.insert(name.clone(), value.clone());
        }
    }

    // Resolve all variable definitions in our operands and replace with that valid constants
    // We do this inplace
    for statement in parsed.iter_mut() {
        match statement {
            Statement::OpcodeStatement(_, _, operands, _) => {
                for operand in operands.iter_mut() {
                    if let Operand::Variable(name) = operand {
                        if let Some(constant) = constants.get(name) {
                            *operand = constant.clone();
                        }
                    }
                }
            }
            _ => {}
        }
    }
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

fn collect_macro_definitions(parsed: &[Statement]) -> HashMap<String, Vec<Statement>> {
    let mut macro_definitions: HashMap<String, Vec<Statement>> = HashMap::new();

    for statement in parsed.iter() {
        if let Statement::MacroStatement(name, statements) = statement {
            macro_definitions.insert(name.clone(), statements.clone());
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
            // We do not count any other definitions in our pc calculations
            _ => {}
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
                    // If it is a jump then we push into the front
                    operands.insert(0, (*resolved_label).into());
                }
            }
            _ => {}
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

    println!("{:?}", instructions);
    instructions
}


    #[test]
    fn simple_test() {
        let input = "
            // Comment
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
            Instruction::new(Opcode::ADD, false, vec![1.into(), 2.into(), 3.into()]),
            Instruction::new(Opcode::JUMP, false, vec![3.into()]),
            Instruction::new(Opcode::SUB, false, vec![1.into(), 2.into(), 3.into()]),
            Instruction::new(Opcode::ADD, false, vec![1.into(), 2.into(), 3.into()]),
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
            Instruction::new(Opcode::ADD, false, vec![1.into(), 2.into(), 3.into()]),
            Instruction::new(Opcode::SUB, false, vec![1.into(), 2.into(), 3.into()]),
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
            Instruction::new(Opcode::ADD, false, vec![1.into(), 2.into(), 3.into()]),
            Instruction::new(Opcode::SUB, false, vec![1.into(), 2.into(), 3.into()]),
            Instruction::new(Opcode::ADD, false, vec![1.into(), 2.into(), 3.into()]),
        ];
        let expected_bytecode = generate_code(expected_instructions);

        let bytecode = compile_asm(input);
        assert_eq!(bytecode, expected_bytecode);
    }

    #[test]
    fn macros_jumping_outside() {
        let input = "
            .macro test {
                add 1 2 3;
                jumpi @label 0;
                sub! 1 2 3;
            };

            $test;
        label:
            add 1 2 3;
        "
        .to_owned();

        let expected_instructions = vec![
            Instruction::new(Opcode::ADD, false, vec![1.into(), 2.into(), 3.into()]),
            Instruction::new(Opcode::JUMPI, false, vec![3.into(), 0.into()]),
            Instruction::new(Opcode::SUB, true, vec![1.into(), 2.into(), 3.into()]),
            Instruction::new(Opcode::ADD, false, vec![1.into(), 2.into(), 3.into()]),
        ];

        let expected_bytecode = generate_code(expected_instructions);
        let bytecode = compile_asm(input);
        assert_eq!(bytecode, expected_bytecode);
    }

    #[test]
    fn tagged_opcodes() {
        let inputs = "
        cast 1 2 3;
        "
        .to_owned();

        let bytecode = compile_asm(inputs);
        let expected_bytecode = "0E000100000000000000020000000000000003";
        assert_eq!(bytecode, expected_bytecode);
    }

    #[test]
    fn tagged_opcodes_as_utypes() {
        let inputs = "
        cast u16 2 3;
        "
        .to_owned();

        let bytecode = compile_asm(inputs);
        let expected_bytecode = "0E000100000000000000020000000000000003";
        assert_eq!(bytecode, expected_bytecode);
    }

    #[test]
    fn set_with_hex_literals() {
        let inputs = "
        set u16 0x1234 2;
        "
        .to_owned();

        let bytecode = compile_asm(inputs);
        let expected_bytecode = "24000112340000000000000002";
        assert_eq!(bytecode, expected_bytecode);
    }

    #[test]
    fn set_with_ff_literal() {
        let inputs = "
        set ff 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46 2;
        "
        .to_owned();

        let bytecode = compile_asm(inputs);
        let expected_bytecode = "24000530644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD460000000000000002";
        assert_eq!(bytecode, expected_bytecode);
    }

    #[test]
    fn test_constants() {
        let inputs = "
        .const c = 0x1234;
        add $c $c $c;
        "
        .to_owned();

        let bytecode = compile_asm(inputs);
        let expected_bytecode = "0000000000000000123400000000000012340000000000001234";
        assert_eq!(bytecode, expected_bytecode);
}


#[test]
fn test_includes_io() {
    // Test includes IO
    let input = std::fs::read_to_string("./test_programs/includes.avm").unwrap();
    let parsed = parse_asm(input);

    // Each file contains just one macro, so we expect that they end up pointing at the same thing
    let expected_length = 2;
    assert_eq!(parsed.len(), expected_length);
}

// Next test: make labels work in the multi file setting