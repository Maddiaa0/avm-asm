use crate::{instruction::Instruction, utils::bytes_to_hex_string};

// Generate code from a string of instructions
pub fn generate_code(instructions: Vec<Instruction>) -> String {
    let mut bytecode = Vec::new();

    // TODO: make sure these are converted to hex bytes accurately
    for instr in instructions {
        instr.append_to_buffer(&mut bytecode);
    }

    bytes_to_hex_string(&bytecode)
}
