use crate::{
    machine::Machine,
    opcode::{OpCode, OpCodeType, Operand},
};

// Intcode instructions come in two parts: an opcode and one or more operands.
// This enum specifies an instruction, which collects the opcode and its
// arguments into a convenient data structure.
#[derive(Debug)]
pub enum Instruction {
    // Add op1 to op2 and store the result at address.
    Add {
        op1:     Operand,
        op2:     Operand,
        address: Operand,
    },
    // Adjust the base pointer.
    AdjustBase(Operand),
    // Halt the program.
    Halt,
    // Read input and store it in memory.
    Input(Operand),
    // Jump to address if condition is false.
    JumpIfFalse {
        condition: Operand,
        address:   Operand,
    },
    // Jump to address if condition is true.
    JumpIfTrue {
        condition: Operand,
        address:   Operand,
    },
    // Multiply op1 with op2 and store the result at address.
    Multiply {
        op1:     Operand,
        op2:     Operand,
        address: Operand,
    },
    // Write output.
    Output(Operand),
    // Test if op1 = op2, and store 1 (true) or 0 (false) at address.
    TestEqual { 
        op1: Operand,
        op2: Operand,
        address: Operand,
    },
    // Test if op1 < op2, and store 1 (true) or 0 (false) at address.
    TestLessThan {
        op1: Operand,
        op2: Operand,
        address: Operand,
    },
}

impl Instruction {
    // Create a new instruction based on the current state of the machine and
    // an opcode.
    pub fn new(machine: &Machine, opcode: OpCode) -> Self {
        // Save the machine's instruction pointer for convenience.
        let ip = machine.get_ip();
        // Destructure the opcode.
        let OpCode {
            opcode_type,
            param1_mode,
            param2_mode,
            param3_mode,
        } = opcode;
        // Match based on the opcode type and return an instruction. The
        // parameters are offset in memory from the current position of the
        // instruction pointer, and the number of arguments depends on the
        // opcode type.
        match opcode_type {
            OpCodeType::Add => Self::Add {
                op1:     Operand::new(param1_mode, machine.read_mem(ip + 1)),
                op2:     Operand::new(param2_mode, machine.read_mem(ip + 2)),
                address: Operand::new(param3_mode, machine.read_mem(ip + 3)),
            },
            OpCodeType::AdjustBase => Self::AdjustBase(
                Operand::new(param1_mode, machine.read_mem(ip + 1)),
            ),
            OpCodeType::Halt => Self::Halt,
            OpCodeType::JumpIfFalse => Self::JumpIfFalse {
                condition: Operand::new(param1_mode, machine.read_mem(ip + 1)),
                address:   Operand::new(param2_mode, machine.read_mem(ip + 2)),
            },
            OpCodeType::JumpIfTrue => Self::JumpIfTrue {
                condition: Operand::new(param1_mode, machine.read_mem(ip + 1)),
                address:   Operand::new(param2_mode, machine.read_mem(ip + 2)),
            },
            OpCodeType::Input => Self::Input(
                Operand::new(param1_mode, machine.read_mem(ip + 1)),
            ),
            OpCodeType::Multiply => Self::Multiply {
                op1:     Operand::new(param1_mode, machine.read_mem(ip + 1)),
                op2:     Operand::new(param2_mode, machine.read_mem(ip + 2)),
                address: Operand::new(param3_mode, machine.read_mem(ip + 3)),
            },
            OpCodeType::Output => Self::Output(
                Operand::new(param1_mode, machine.read_mem(ip + 1)),
            ),
            OpCodeType::TestLessThan => Self::TestLessThan {
                op1:     Operand::new(param1_mode, machine.read_mem(ip + 1)),
                op2:     Operand::new(param2_mode, machine.read_mem(ip + 2)),
                address: Operand::new(param3_mode, machine.read_mem(ip + 3)),
            },
            OpCodeType::TestEqual => Self::TestEqual {
                op1:     Operand::new(param1_mode, machine.read_mem(ip + 1)),
                op2:     Operand::new(param2_mode, machine.read_mem(ip + 2)),
                address: Operand::new(param3_mode, machine.read_mem(ip + 3)),
            },
        }
    }
}
