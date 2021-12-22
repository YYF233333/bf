use std::io::*;

use crate::parser::*;
use Instruction::*;

const SIZE: usize = 30000;

#[derive(Debug, Clone)]
pub struct VM {
    insts: [Instruction; SIZE],
    mem: [u8; SIZE as usize],
    pointer: usize,
    pc: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            insts: [Undefined; SIZE],
            mem: [0; SIZE as usize],
            pointer: 0,
            pc: 0,
        }
    }

    pub fn init(&mut self, insts: Vec<Instruction>) {
        for i in 0..insts.len() {
            self.insts[i] = insts[i];
        }
    }

    fn store_memory(&mut self, val: u8) {
        self.mem[self.pointer] = val;
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
            let n = n as usize;
            if n <= self.mem.len() && self.pointer < self.mem.len() - n {
                self.pointer += n;
            } else {
                panic!("Pointer overflow");
            }
        } else if n < 0 {
            let n = n.abs() as usize;
            if self.pointer >= n {
                self.pointer -= n;
            } else {
                panic!("Pointer underflow");
            }
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
        while self.pc < SIZE {
            match self.insts[self.pc] {
                Add(n) => {
                    self.add(n);
                }
                Move(n) => {
                    self.move_ptr(n);
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
