use crate::opcodes::Opcode;



// An instruction is a pairing of an opcode and its operands.
#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<u64>,
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: Vec<u64>) -> Self {
        Instruction { opcode, operands }
    }
}