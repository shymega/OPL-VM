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
    /// Move (MOV) - move a value to a register.
    /// Accepts two values: register ID, value (as `u8` array)
    /// Register ID example: `oax`.
    MOV = 0x3(i16, &[u8]),
    /// GRAb - dynamically grab a variable from a register.
    /// Accepts one value - register ID - such as `oax`
    GRB = 0x4(i16),
}

impl Default for Opcode {
    fn default() -> Self {
        Self::NOP
    }
}
