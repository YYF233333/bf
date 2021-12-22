extern crate btree_graph;
extern crate nom;

use std::{fs, time::Instant};

mod parser;

use parser::*;
mod optimization;

mod interpreter;
use interpreter::*;
mod macro_inst;
use macro_inst::*;

fn main() {
    let text = fs::read_to_string("test.bf").unwrap();
    let insts = parse(text.as_str()).unwrap().1;
    
    let bb = split_basicblocks(insts);
    //let bb = opt_copy_mult(bb);
    let insts = bb.into_iter().flatten().collect();
    let mut insts = merge_opcode(insts);
    //println!("{:?}", insts);
    let mut stack = vec![];
    for idx in 0..insts.len() {
        match insts[idx] {
            Instruction::Set => {
                stack.push(idx);
            }
            Instruction::Test => {
                let set = stack.pop().unwrap();
                insts[set] = Instruction::JmpZero(idx + 1);
                insts[idx] = Instruction::JmpNonZero(set + 1);
            }
            _ => (),
        }
    }
    let mut vm = VM::new();
    vm.init(insts);
    let start = Instant::now();
    vm.run();
    println!("time cost: {}", start.elapsed().as_secs_f32());
}


