use crate::{instruction::Instruction, utils::bytes_to_hex_string};


// Generate code from a string of instructions
pub fn generate_code(instructions: Vec<Instruction>) -> String {
    let mut bytecode = Vec::new();

    // TODO: make sure these are converted to hex bytes accurately
    for instr in instructions {
        bytecode.push(instr.opcode as u8);
        for operand in instr.operands {
            bytecode.extend(operand.to_be_bytes());
        }
    }

    bytes_to_hex_string(&bytecode)
}