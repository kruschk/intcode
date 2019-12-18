#[derive(Debug)]
pub enum Mode {
    AddressMode,
    ImmediateMode,
    RelativeMode,
}

#[derive(Debug)]
pub struct OpCode {
    pub opcode_type: OpCodeType,
    pub param1_mode: Mode,
    pub param2_mode: Mode,
    pub param3_mode: Mode,
}

impl OpCode {
    pub fn new(mut opcode: isize) -> Result<OpCode, ()> {
        let opcode_type = match opcode%100 {
             1 => OpCodeType::Add,
             2 => OpCodeType::Multiply,
             3 => OpCodeType::Input,
             4 => OpCodeType::Output,
             5 => OpCodeType::JumpIfTrue,
             6 => OpCodeType::JumpIfFalse,
             7 => OpCodeType::TestLessThan,
             8 => OpCodeType::TestEqual,
             9 => OpCodeType::AdjustBase,
            99 => OpCodeType::Halt,
             _ => return Err(()),
        };
        opcode /= 100;
        let param1_mode = match opcode % 10 {
            0 => Mode::AddressMode,
            1 => Mode::ImmediateMode,
            2 => Mode::RelativeMode,
            _ => return Err(()),
        };
        opcode /= 10;
        let param2_mode = match opcode % 10 {
            0 => Mode::AddressMode,
            1 => Mode::ImmediateMode,
            2 => Mode::RelativeMode,
            _ => return Err(()),
        };
        opcode /= 10;
        let param3_mode = match opcode % 10 {
            0 => Mode::AddressMode,
            1 => Mode::ImmediateMode,
            2 => Mode::RelativeMode,
            _ => return Err(()),
        };
        Ok(Self {
            opcode_type,
            param1_mode,
            param2_mode,
            param3_mode,
        })
    }
}

#[derive(Debug)]
pub enum OpCodeType {
    Add,
    AdjustBase,
    Halt,
    Input,
    JumpIfFalse,
    JumpIfTrue,
    Output,
    Multiply,
    TestEqual,
    TestLessThan,
}

// Intcode instructions come in two parts: an opcode and one or more operands.
// This enum specifies an operand, which can be in in Address, Immediate, or
// Relative mode, based on the form of the opcode.
#[derive(Debug)]
pub enum Operand {
    Address(isize),
    Immediate(isize),
    Relative(isize),
}

impl Operand {
    // Create a new operand given a Mode and an argument.
    pub fn new(mode: Mode, argument: isize) -> Operand {
        match mode {
            Mode::AddressMode   => Self::Address(argument),
            Mode::ImmediateMode => Self::Immediate(argument),
            Mode::RelativeMode  => Self::Relative(argument),
        }
    }
}
