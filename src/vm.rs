//! This is the core of the `OPL` VM.

#[derive(Debug, Default)]
pub struct VM {
    registers: [i32; 32],
    program: Vec<u8>,
    program_counter: usize,
}

impl VM {
    #[must_use]
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: Vec::new(),
            program_counter: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();

        assert_eq!(test_vm.registers[0], 0);
        assert_eq!(test_vm.program, []);
        assert_eq!(test_vm.program_counter, 0);
    }
}
