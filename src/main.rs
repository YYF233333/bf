extern crate brainfuck;
extern crate btree_graph;
extern crate nom;

use std::{
    fs,
    io::{stdin, stdout, Read},
};

mod parser;
use brainfuck::tape::ArrayTape;
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
    let mut vm = VM::new();
    vm.init(insts);
    vm.run();
    /*let mut stdin = stdin();
    let mut stdout = stdout();
    let program = brainfuck::program::Program::parse(text.as_str()).unwrap();
    let mut interp = brainfuck::Interpreter::<ArrayTape>::new(program, &mut stdin, &mut stdout);
    println!("{:?}", interp.run());*/
}
