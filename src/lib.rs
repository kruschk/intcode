use std::{
    fs,
    io,
    path::Path,
};

mod instruction;
mod opcode;

use instruction::{Instruction, Operand};
use opcode::OpCode;

pub fn execute_intcode_program(mem: &mut[isize]) -> isize {
    let mut ip = 0;
    loop {
        let opcode = OpCode::new(mem[ip]).expect("Error generating an opcode.");
        let instruction = Instruction::new(mem, ip, opcode);
        match instruction {
            Instruction::Add { op1, op2, address } => {
                let op1 = match op1 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let op2 = match op2 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => p,
                    Operand::Immediate(_) => panic!("The third operand of an \
                        Add instruction cannot use immediate mode."),
                };
                mem[address] = op1 + op2;
                ip += 4;
            },
            Instruction::Halt => break,
            Instruction::Input(op1) => {
                let mut input = String::new();
                if let Err(e) = io::stdin().read_line(&mut input) {
                    panic!(e);
                }
                match op1 {
                    Operand::Address(p) => {
                        mem[p] = input.trim().parse().expect("Error parsing \
                            input");
                    },
                    Operand::Immediate(_) => panic!("An Input instruction's \
                        operand cannot use immediate mode."),
                };
                ip += 2;
            },
            Instruction::JumpIfFalse { condition, address } => {
                let condition = match condition {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                if 0 == condition {
                    ip = address as usize;
                } else {
                    ip += 3;
                }
            },
            Instruction::JumpIfTrue { condition, address } => {
                let condition = match condition {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                if 0 != condition {
                    ip = address as usize;
                } else {
                    ip += 3;
                }
            },
            Instruction::Multiply { op1, op2, address } => {
                let op1 = match op1 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let op2 = match op2 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => p,
                    Operand::Immediate(_) => panic!("The third operand of a \
                        Multiply instruction cannot use immediate mode."),
                };
                mem[address] = op1*op2;
                ip += 4;
            },
            Instruction::Output(op1) => {
                match op1 {
                    Operand::Address(p) => {
                        println!("{}", mem[p]);
                    },
                    Operand::Immediate(v) => {
                        println!("{}", v);
                    },
                };
                ip += 2;
            },
            Instruction::TestEqual { op1, op2, address } => {
                let op1 = match op1 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let op2 = match op2 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => p,
                    Operand::Immediate(_) => panic!("The third operand of a \
                        TestEqual instruction cannot use immediate mode."),
                };
                if op1 == op2 {
                    mem[address] = 1;
                } else {
                    mem[address] = 0;
                }
                ip += 4;
            },
            Instruction::TestLessThan { op1, op2, address } => {
                let op1 = match op1 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let op2 = match op2 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => p,
                    Operand::Immediate(_) => panic!("The third operand of a \
                        TestEqual instruction cannot use immediate mode."),
                };
                if op1 < op2 {
                    mem[address] = 1;
                } else {
                    mem[address] = 0;
                }
                ip += 4;
            },
        }
    }
    //println!("{:?}", mem);
    mem[0]
}

pub fn find_noun_and_verb(src: &[isize]) -> Option<(isize, isize)> {
    const MAGIC_NUM: isize = 19_690_720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut src_copy = vec![0; src.len()];
            src_copy.copy_from_slice(src);
            src_copy[1] = noun;
            src_copy[2] = verb;
            if MAGIC_NUM == execute_intcode_program(&mut src_copy) {
                return Some((noun, verb));
            }
        }
    }
    None
}

pub fn parse_source(path: &Path) -> io::Result<Vec<isize>> {
    let src = fs::read_to_string(path)?;
    Ok(src.trim()
       .split(',')
       .map(|elem| elem.parse().expect("Error reading an integer in the source \
            file."))
       .collect())
}
