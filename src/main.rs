extern crate btree_graph;
extern crate nom;

use std::{fs};

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
    let insts = graph.to_insts();
    let mut insts = replace(insts, vec![Instruction::Set, Instruction::Add(-1), Instruction::Test], Instruction::Clear);
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

fn replace(mut insts: Vec<Instruction>, pat: Vec<Instruction>, replace_with: Instruction) -> Vec<Instruction> {
    let mut idx = 0;
    while idx < insts.len() {
        if insts[idx] == pat[0] {
            let mut flag = true;
            for j in 1..pat.len() {
                if insts[idx+j] != pat[j] {flag = false; break;}
            }
            if flag {
                for i in 0..pat.len() {
                    insts.remove(idx);
                }
                insts.insert(idx, replace_with);
            }
        }
        idx += 1;
    }
    insts
}