//! Module of OPL-VM opcodes.

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code, clippy::upper_case_acronyms, non_camel_case_types)]
pub enum Opcode {
    /// Halt virtual machine.
    HLT = 0x0,
    /// Illegal instruction.
    IGL = 0x1,
    /// No Operation - do nothing.
    NOP = 0x2,
    /// STOre Integer - store a 16-bit integer in a register. 
    /// Accepts two values, an 16-bit integer and a register ID.
    /// Registry ID example: `oax`.
    STO = 0x3,
    /// GRAb - dynamically grab a variable from a register. 
    /// Accepts one value - register ID - such as `oax`
    GRA = 0x4,
}

impl Default for Opcode {
    fn default() -> Self {
        Self::NOP
    }
}
