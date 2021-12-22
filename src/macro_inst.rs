use crate::parser::Instruction;


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