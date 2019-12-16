#[derive(Debug)]
pub enum Mode {
    AddressMode,
    ImmediateMode,
}

#[derive(Debug)]
pub struct OpCode {
    pub opcode_type: OpCodeType,
    pub parameter1_mode: Mode,
    pub parameter2_mode: Mode,
    pub parameter3_mode: Mode,
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
            99 => OpCodeType::Halt,
            _ => return Err(()),
        };
        opcode /= 100;
        let parameter1_mode = match opcode % 10 {
            0 => Mode::AddressMode,
            1 => Mode::ImmediateMode,
            _ => return Err(()),
        };
        opcode /= 10;
        let parameter2_mode = match opcode % 10 {
            0 => Mode::AddressMode,
            1 => Mode::ImmediateMode,
            _ => return Err(()),
        };
        opcode /= 10;
        let parameter3_mode = match opcode % 10 {
            0 => Mode::AddressMode,
            1 => Mode::ImmediateMode,
            _ => return Err(()),
        };
        Ok(Self {
            opcode_type,
            parameter1_mode,
            parameter2_mode,
            parameter3_mode,
        })
    }
}

#[derive(Debug)]
pub enum OpCodeType {
    Add,
    Halt,
    Input,
    JumpIfFalse,
    JumpIfTrue,
    Output,
    Multiply,
    TestEqual,
    TestLessThan,
}
