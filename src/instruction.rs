use crate::opcode::{Mode, OpCode, OpCodeType};

#[derive(Debug)]
pub enum Instruction {
    Add {
        op1:     Operand,
        op2:     Operand,
        address: Operand,
    },
    Halt,
    Input(Operand),
    JumpIfFalse {
        condition: Operand,
        address:   Operand,
    },
    JumpIfTrue {
        condition: Operand,
        address:   Operand,
    },
    Multiply {
        op1:     Operand,
        op2:     Operand,
        address: Operand,
    },
    Output(Operand),
    TestEqual {
        op1: Operand,
        op2: Operand,
        address: Operand,
    },
    TestLessThan {
        op1: Operand,
        op2: Operand,
        address: Operand,
    },
}

impl Instruction {
    pub fn new(mem: &[isize], ip: usize, opcode: OpCode) -> Instruction {
        let OpCode {
            opcode_type,
            parameter1_mode,
            parameter2_mode,
            parameter3_mode,
        } = opcode;
        match opcode_type {
            OpCodeType::Add => Instruction::Add {
                op1:     Operand::new(parameter1_mode, mem[ip + 1]),
                op2:     Operand::new(parameter2_mode, mem[ip + 2]),
                address: Operand::new(parameter3_mode, mem[ip + 3]),
            },
            OpCodeType::Halt => Instruction::Halt,
            OpCodeType::JumpIfFalse => Instruction::JumpIfFalse {
                condition: Operand::new(parameter1_mode, mem[ip + 1]),
                address:   Operand::new(parameter2_mode, mem[ip + 2]),
            },
            OpCodeType::JumpIfTrue => Instruction::JumpIfTrue {
                condition: Operand::new(parameter1_mode, mem[ip + 1]),
                address:   Operand::new(parameter2_mode, mem[ip + 2]),
            },
            OpCodeType::Input => Instruction::Input(
                Operand::new(parameter1_mode, mem[ip + 1]),
            ),
            OpCodeType::Multiply => Instruction::Multiply {
                op1:     Operand::new(parameter1_mode, mem[ip + 1]),
                op2:     Operand::new(parameter2_mode, mem[ip + 2]),
                address: Operand::new(parameter3_mode, mem[ip + 3]),
            },
            OpCodeType::Output => Instruction::Output(
                Operand::new(parameter1_mode, mem[ip + 1]),
            ),
            OpCodeType::TestLessThan => Instruction::TestLessThan {
                op1:     Operand::new(parameter1_mode, mem[ip + 1]),
                op2:     Operand::new(parameter2_mode, mem[ip + 2]),
                address: Operand::new(parameter3_mode, mem[ip + 3]),
            },
            OpCodeType::TestEqual => Instruction::TestEqual {
                op1:     Operand::new(parameter1_mode, mem[ip + 1]),
                op2:     Operand::new(parameter2_mode, mem[ip + 2]),
                address: Operand::new(parameter3_mode, mem[ip + 3]),
            },
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Address(usize),
    Immediate(isize),
}

impl Operand {
    pub fn new(mode: Mode, argument: isize) -> Operand {
        match mode {
            Mode::AddressMode => Self::Address(argument as usize),
            Mode::ImmediateMode => Self::Immediate(argument),
        }
    }
}
