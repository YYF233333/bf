extern crate brainfuck;
extern crate btree_graph;
extern crate nom;

use std::fs;

mod parser;

use parser::*;
mod optimization;
use optimization::*;
mod interpreter;
use interpreter::*;

fn main() {
    let text = fs::read_to_string("test.bf").unwrap();
    let insts = parse(text.as_str()).unwrap().1;
    let graph = FlowGraph::from_insts(insts);
    let mut insts = graph.to_insts();
    let mut stack = vec![];
    for idx in 0..insts.len() {
        match insts[idx] {
            Instruction::Set => {
                stack.push(idx);
            },
            Instruction::Test => {
                let set = stack.pop().unwrap();
                insts[set] = Instruction::JmpZero(idx+1);
                insts[idx] = Instruction::JmpNonZero(set+1);
            },
            _ => (),
        }
    }
    let mut vm = VM::new();
    vm.init(insts);
    vm.run();
}
