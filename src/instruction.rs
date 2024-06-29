use crate::opcodes::Opcode;

// An instruction is a pairing of an opcode and its operands.
#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub indirect: bool,
    pub operands: Vec<u64>,
}

impl Instruction {
    pub fn new(opcode: Opcode, indirect: bool, operands: Vec<u64>) -> Self {
        Instruction {
            opcode,
            indirect,
            operands,
        }
    }

    // Append the instruction to a buffer, todo: probably best as a trait?
    // TODO: more granularity on a per opcode basis? ones that do not have indirect are not supported
    pub fn append_to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.opcode as u8);
        buffer.push(self.indirect as u8);
        for operand in &self.operands {
            buffer.extend(operand.to_be_bytes());
        }
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            opcode: Opcode::ADD,
            indirect: false,
            operands: Vec::new(),
        }
    }
}
