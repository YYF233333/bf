use std::io::*;

use crate::parser::*;
use Instruction::*;

const SIZE: i32 = 30000;

#[derive(Debug, Clone)]
pub struct VM {
    insts: Vec<Instruction>,
    stack: Vec<usize>,
    mem: [u8; SIZE as usize],
    pointer: i32,
    pc: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            insts: vec![],
            stack: vec![],
            mem: [0; SIZE as usize],
            pointer: 0,
            pc: 0,
        }
    }

    pub fn init(&mut self, insts: Vec<Instruction>) {
        self.insts = insts;
    }

    pub fn run(&mut self) {
        eprintln!("check");
        while let inst = self.insts[self.pc] {
            match inst {
                Add(i) => {
                    eprintln!("Add");
                    let mut res = self.mem[self.pointer as usize] as i32 + i;
                    while res < 0 || res >= 256 {
                        if res < 0 {
                            res += 256;
                        } else if res >= 256 {
                            res -= 256;
                        }
                    }
                    eprintln!("res = {}", res);
                    assert!(res >= 0 && res < 256);
                    self.mem[self.pointer as usize] = res as u8;
                }
                Move(i) => {
                    eprintln!("Move");
                    self.pointer += i;
                    if self.pointer >= SIZE {
                        self.pointer -= SIZE;
                    } else if self.pointer < 0 {
                        self.pointer += SIZE;
                    }
                    eprintln!("pointer = {}", self.pointer);
                }
                Input => {
                    self.mem[self.pointer as usize] = stdin().bytes().next().unwrap().unwrap();
                }
                Output => {
                    print!("{}", self.mem[self.pointer as usize] as char);
                }
                Set => {
                    eprintln!("Set");
                    if self.mem[self.pointer as usize] == 0 {
                        while self.insts[self.pc] != Test {
                            self.pc += 1;
                        }
                    } else {
                        self.stack.push(self.pc);
                    }
                }
                Test => {
                    eprintln!("Test");
                    if self.mem[self.pointer as usize] == 0 {
                        self.stack.pop().unwrap();
                    } else {
                        self.pc = *self.stack.last().unwrap();
                    }
                }
                STOP => {
                    break;
                }
            }
            self.pc += 1;
            eprintln!("PC = {}", self.pc);
        }
    }
}
