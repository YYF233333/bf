use std::io::*;

use crate::parser::*;
use Instruction::*;

const SIZE: usize = 30000;

#[derive(Debug, Clone)]
pub struct VM {
    insts: Vec<Instruction>,
    mem: [u8; SIZE],
    pointer: usize,
    pc: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            insts: vec![],
            mem: [0; SIZE as usize],
            pointer: 0,
            pc: 0,
        }
    }

    pub fn init(&mut self, insts: Vec<Instruction>) {
        self.insts = insts;
    }

    fn store_memory(&mut self, val: u8) {
        self.mem[self.pointer] = val;
    }

    fn store_with_offset(&mut self, offset: i32, val: u8) {
        if offset >= 0 {
            self.mem[self.pointer + offset as usize] = val;
        } else {
            self.mem[self.pointer - offset.abs() as usize] = val;
        }
    }

    fn load_memory(&self) -> u8 {
        self.mem[self.pointer]
    }

    fn add(&mut self, n: i32) {
        let value = self.load_memory();
        self.store_memory(if n > 0 {
            value.wrapping_add(n as u8)
        } else if n < 0 {
            value.wrapping_sub(n.abs() as u8)
        } else {
            value
        });
    }

    fn move_ptr(&mut self, n: i32) {
        if n > 0 {
            self.pointer += n as usize;
        } else if n < 0 {
            self.pointer -= n.abs() as usize;
        }
        if self.pointer >= SIZE {
            panic!("Pointer Overflow");
        }
    }

    fn getchar(&mut self) {
        self.store_memory(stdin().bytes().next().unwrap().unwrap());
    }

    fn putchar(&mut self) {
        print!("{}", self.load_memory() as char);
    }

    fn jmp_zero(&mut self, idx: usize) {
        if self.load_memory() == 0 {
            self.pc = idx - 1;
        }
    }

    fn jmp_nonzero(&mut self, idx: usize) {
        if self.load_memory() != 0 {
            self.pc = idx - 1;
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.insts[self.pc] {
                Add(n) => {
                    self.add(n);
                }
                Move(n) => {
                    self.move_ptr(n);
                }
                AddMove(m, n) => {
                    self.add(m);
                    self.move_ptr(n);
                }
                Mul(offset, power) => {
                    eprintln!("offset: {} power: {}", offset, power);
                    let val = self.load_memory();
                    if power > 0 {
                        self.store_with_offset(offset, val.wrapping_mul(power as u8));
                    } else if power < 0 {
                        let val = val.wrapping_mul(power.abs() as u8);
                        let val = 256 - val as u32;
                        self.store_with_offset(offset, val as u8);
                    } else {
                        self.store_with_offset(offset, 0);
                    }
                }
                Clear => {
                    self.store_memory(0);
                }
                Input => {
                    self.getchar();
                }
                Output => {
                    self.putchar();
                }
                JmpNonZero(idx) => {
                    self.jmp_nonzero(idx);
                }
                JmpZero(idx) => {
                    self.jmp_zero(idx);
                }
                STOP => {
                    eprintln!("STOP");
                    break;
                }
                _ => panic!("{}", format!("Unexpected Inst {:?}", self.insts[self.pc])),
            }
            self.pc += 1;
        }
    }
}
