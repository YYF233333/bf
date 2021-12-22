use std::{collections::{BTreeMap}};

use crate::{parser::Instruction::{self, *}, optimization::BasicBlock};

pub fn split_basicblocks(insts: Vec<Instruction>) -> Vec<BasicBlock> {
    let mut bb = vec![];
    let mut id = 0;
    let mut cur_bb = BasicBlock::new(id);
    for inst in insts {
        match inst {
            Set => {
                bb.push(cur_bb);
                id += 1;
                cur_bb = BasicBlock::new(id);
                cur_bb.push(inst);
            }
            Test => {
                cur_bb.push(inst);
                bb.push(cur_bb);
                id += 1;
                cur_bb = BasicBlock::new(id);
            }
            _ => cur_bb.push(inst),
        }
    }
    if !cur_bb.is_empty() {bb.push(cur_bb);}
    bb
}

pub fn opt_copy_mult(mut bb: Vec<BasicBlock>) -> Vec<BasicBlock> {
    for bb in &mut bb {
        if bb.is_leafloop() && !bb.contains_io() {
            println!("{:?}", bb.insts());
            let mut mem = BTreeMap::new();
            let mut idx = 0;
            for inst in bb.insts() {
                match inst {
                    Set => (),
                    Add(n) => {
                        let t = mem.get(&idx);
                        let v = match t {
                            Some(t) => *t + n,
                            None => n
                        };
                        mem.insert(idx, v);
                    }
                    Move(n) => {idx += n;}
                    //Clear => {mem.insert(idx, 0);}
                    Test => break,
                    _ => panic!("Unexpected Inst {:?}", inst),
                }
            }
            if !(idx == 0 && mem.get(&idx) == Some(&-1)) { continue; }
            assert!(bb.last() == Some(&Test));
            bb.clear();
            for (k, v) in mem {
                if k != 0 {
                    bb.push(Mul(k, v));
                }
            }
            bb.push(Clear);
            println!("{:?}", bb.insts());
        }
    }
    bb
}

pub fn merge_opcode(mut insts: Vec<Instruction>) -> Vec<Instruction> {
    let mut i = 0;
    while i < insts.len() {
        match insts[i] {
            Instruction::Add(m) => match insts[i + 1] {
                Instruction::Move(n) => {
                    insts.remove(i);
                    insts.remove(i);
                    insts.insert(i, Instruction::AddMove(m, n));
                }
                _ => (),
            },
            _ => (),
        }
        i += 1;
    }
    insts
}

pub fn replace(
    mut insts: Vec<Instruction>,
    pat: Vec<Instruction>,
    replace_with: Instruction,
) -> Vec<Instruction> {
    let mut idx = 0;
    while idx < insts.len() {
        if insts[idx] == pat[0] {
            let mut flag = true;
            for j in 1..pat.len() {
                if insts[idx + j] != pat[j] {
                    flag = false;
                    break;
                }
            }
            if flag {
                for _ in 0..pat.len() {
                    insts.remove(idx);
                }
                insts.insert(idx, replace_with);
            }
        }
        idx += 1;
    }
    insts
}