//! Bytecode instructions for the `OPL` VM.

use crate::opcode::Opcode;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    #[must_use]
    pub fn new(opcode: Opcode) -> Self {
        Self { opcode }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, opcode::Opcode};

    #[test]
    fn test_hlt_opcode() {
        let opcode = Opcode::HLT;

        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_hlt_instruction() {
        let instruction = Instruction::new(Opcode::HLT);

        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
