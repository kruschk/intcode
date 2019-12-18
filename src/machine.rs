use crate::instruction::Instruction;
use crate::opcode::{ OpCode, Operand };
use std::{
    convert::{ TryFrom },
    fs,
    io::{BufRead, BufReader, Read, self, Write},
    path::Path,
};

// An Intcode machine is represented by a base pointer, an instruction pointer,
// and memory. These fields may all mutate during Intcode program execution.
#[derive(Debug)]
pub struct Machine {
    bp:  isize,      // base pointer
    ip:  isize,      // instruction pointer
    mem: Vec<isize>, // memory
}

impl Machine {
    // Convert an operand into the value at a memory address (address mode), a
    // literal value (immediate mode), or the value at a memory address offset
    // from the Machine's base pointer (relative mode).
    fn convert_operand(&mut self, operand: Operand) -> isize {
        match operand {
            Operand::Address(p)   => self.read_mem(p),
            Operand::Immediate(v) => v,
            Operand::Relative(o)  => self.read_mem(self.bp + o),
        }
    }

    // This is necessary because address operands are often written in immediate
    // mode, but the opcode specifies address Mode for the operand. I think it's
    // an error in the specification, but still workable since we know how
    // instructions are structured in memory.
    fn convert_address(&self, address: Operand) -> isize {
        match address {
            Operand::Address(p) => p,
            Operand::Immediate(v) => v,
            Operand::Relative(o) => self.bp + o,
        }
    }

    // Initialize a new machine from source code. The base pointer and
    // instruction pointer will always initialize to 0.
    pub fn new(src: &str) -> Self {
        let mem = src.trim()
            .split(',')
            .map(|elem| elem.parse()
                .expect("Error reading an integer in the source file."))
            .collect();
        Self {
            bp: 0,
            ip: 0,
            mem,
        }
    }

    // Initialize a new machine from a source code file. This static method is
    // just a convenient wrapper for the `new` static method.
    pub fn new_from_file(src_filename: &Path) -> io::Result<Self> {
        let src = fs::read_to_string(src_filename)?;
        Ok(Self::new(&src))
    }

    // Execute a program that has been loaded into memory.
    pub fn execute<R: Read, W: Write>(&mut self, reader: R, mut writer: W)
        -> io::Result<isize> {
            let mut reader = BufReader::new(reader);
            loop {
                //self.write_state(io::stderr());
                let opcode = OpCode::new(self.read_mem(self.ip))
                    .expect("Error generating an opcode.");
                let instruction = Instruction::new(&self, opcode);
                //eprintln!("{}", self.bp);
                //eprintln!("{:?}", instruction);
                match instruction {
                    Instruction::Add { op1, op2, address } => {
                        let op1 = self.convert_operand(op1);
                        let op2 = self.convert_operand(op2);
                        let address = self.convert_address(address);
                        self.write_mem(address, op1 + op2);
                        self.ip += 4;
                    },
                    Instruction::AdjustBase(operand) => {
                        let operand = self.convert_operand(operand);
                        self.bp += operand;
                        self.ip += 2;
                    },
                    Instruction::Halt => break,
                    Instruction::Input(operand) => {
                        let mut input = String::new();
                        if let Err(e) = reader.read_line(&mut input) {
                            panic!(e);
                        }
                        match operand {
                            Operand::Address(p) => {
                                let input = input.trim()
                                    .parse()
                                    .expect("Error parsing input.");
                                self.write_mem(p, input);
                            },
                            Operand::Immediate(_) => panic!("An Input \
                                instruction's operand cannot use immediate \
                                mode."),
                            Operand::Relative(o) => {
                                let input = input.trim()
                                    .parse()
                                    .expect("Error parsing input.");
                                self.write_mem(self.bp + o, input);
                            },
                        };
                        self.ip += 2;
                    },
                    Instruction::JumpIfFalse { condition, address } => {
                        let condition = self.convert_operand(condition);
                        let address = self.convert_operand(address);
                        if 0 == condition {
                            self.ip = address;
                        } else {
                            self.ip += 3;
                        }
                    },
                    Instruction::JumpIfTrue { condition, address } => {
                        let condition = self.convert_operand(condition);
                        let address = self.convert_operand(address);
                        if 0 != condition {
                            self.ip = address;
                        } else {
                            self.ip += 3;
                        }
                    },
                    Instruction::Multiply { op1, op2, address } => {
                        let op1 = self.convert_operand(op1);
                        let op2 = self.convert_operand(op2);
                        let address = self.convert_address(address);
                        self.write_mem(address, op1*op2);
                        self.ip += 4;
                    },
                    Instruction::Output(operand) => {
                        match operand {
                            Operand::Address(p) => {
                                writeln!(writer, "{}", self.read_mem(p))?;
                            },
                            Operand::Immediate(v) => {
                                writeln!(writer, "{}", v)?;
                            },
                            Operand::Relative(o) => {
                                let address = self.bp + o;
                                writeln!(writer, "{}", self.read_mem(address))?;
                            },
                        };
                        self.ip += 2;
                    },
                    Instruction::TestEqual { op1, op2, address } => {
                        let op1 = self.convert_operand(op1);
                        let op2 = self.convert_operand(op2);
                        let address = self.convert_address(address);
                        if op1 == op2 {
                            self.write_mem(address, 1);
                        } else {
                            self.write_mem(address, 0);
                        }
                        self.ip += 4;
                    },
                    Instruction::TestLessThan { op1, op2, address } => {
                        let op1 = self.convert_operand(op1);
                        let op2 = self.convert_operand(op2);
                        let address = self.convert_address(address);
                        if op1 < op2 {
                            self.write_mem(address, 1);
                        } else {
                            self.write_mem(address, 0);
                        }
                        self.ip += 4;
                    },
                }
            }
            writer.flush()?;
            Ok(self.read_mem(0))
        }

    // Dump the machine's memory.
    pub fn dump(&self) -> &[isize] { &self.mem }

    pub fn get_bp(&self) -> isize { self.bp }

    pub fn get_ip(&self) -> isize { self.ip }

    pub fn read_mem(&self, address: isize) -> isize {
        let address = usize::try_from(address).expect("Negtaive address");
        if self.mem.len() <= address {
            0
        } else {
            self.mem[address]
        }
    }

    pub fn write_mem(&mut self, address: isize, value: isize) {
        let address = usize::try_from(address).expect("Negtaive address");
        if self.mem.len() <= address {
            self.mem.resize(2*(address + 1), 0);
        }
        self.mem[address] = value;
    }

    pub fn write_state<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writeln!(writer, "Memory: {:?}", self.mem)?;
        writeln!(writer, "Base pointer: {}", self.bp)?;
        writeln!(writer, "Instruction pointer: {}", self.ip)?;
        Ok(())
    }
}
