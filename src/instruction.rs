use crate::{opcodes::Opcode, parser::Operand, parser::TypeTag};

// An instruction is a pairing of an opcode and its operands.
#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub indirect: bool,
    pub operands: Vec<Operand>,
}

impl Instruction {
    pub fn new(opcode: Opcode, indirect: bool, operands: Vec<Operand>) -> Self {
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

        match self.opcode {
            // Opcodes that contain tags will push their first opcode (the tag)
            // as a u8
            Opcode::CAST | Opcode::SET => {
                // yuck
                let mut ops = self.operands.clone();

                let first_operand = ops.remove(0);
                let tag = match first_operand {
                    Operand::Decimal(tag) => tag as u8,
                    Operand::Tag(tag) => tag as u8,
                    _ => panic!("YOU DONKEY: First operand of CAST or SET must be a tag"),
                };

                // First operand MUST be a numeric, i.e. the tag
                buffer.push(tag);

                // TODO: support set for large types
                for operand in &ops {
                    buffer.extend(operand.to_be_bytes());
                }
            }
            _ => {
                for operand in &self.operands {
                    buffer.extend(operand.to_be_bytes());
                }
            }
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
