use btree_graph::{AddEdge, AddVertex, BTreeGraph};
use std::collections::BTreeMap;

use crate::parser::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BasicBlock {
    id: i32,
    insts: Vec<Instruction>,
}

impl BasicBlock {
    pub fn new(id: i32) -> Self {
        Self { id, insts: vec![] }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn is_empty(&self) -> bool {
        self.insts.is_empty()
    }

    pub fn push(&mut self, inst: Instruction) {
        match inst {
            Instruction::Add(i) => {
                if let Some(Instruction::Add(j)) = self.insts.last_mut() {
                    *j += i;
                    return;
                }
            }
            Instruction::Move(i) => {
                if let Some(Instruction::Move(j)) = self.insts.last_mut() {
                    *j += i;
                    return;
                }
            }
            _ => (),
        }
        self.insts.push(inst);
    }
}

impl From<Vec<Instruction>> for BasicBlock {
    fn from(insts: Vec<Instruction>) -> Self {
        Self { id: 0, insts }
    }
}

impl From<BasicBlock> for Vec<Instruction> {
    fn from(bb: BasicBlock) -> Self {
        bb.insts
    }
}

#[derive(Debug)]
pub struct FlowGraph {
    graph: BTreeGraph<i32, i32>,
    bb: BTreeMap<i32, BasicBlock>,
}

impl FlowGraph {
    pub fn from_insts(insts: Vec<Instruction>) -> Self {
        let mut graph = BTreeGraph::new();
        let mut bb = BTreeMap::new();
        let mut cur_bb = BasicBlock::new(0);
        for inst in &insts {
            cur_bb.push(*inst);
            if let Instruction::Set | Instruction::Test = inst {
                let next_bb = BasicBlock::new(cur_bb.id() + 1);
                graph.add_vertex(cur_bb.id());
                bb.insert(cur_bb.id(), cur_bb);
                cur_bb = next_bb;
            }
        }
        if !cur_bb.is_empty() {
            graph.add_vertex(cur_bb.id());
            bb.insert(cur_bb.id(), cur_bb);
        }

        let mut cur_id = 0;
        let mut edge_id = 0;
        let mut stack = vec![];
        for inst in insts {
            match inst {
                Instruction::Set => {
                    stack.push(cur_id);
                    graph.add_edge(cur_id, cur_id + 1, edge_id);
                    cur_id += 1;
                    edge_id += 1;
                }
                Instruction::Test => {
                    let parent_id = stack.pop().unwrap();
                    graph.add_edge(cur_id, cur_id + 1, edge_id);
                    graph.add_edge(parent_id, cur_id + 1, edge_id + 1);
                    graph.add_edge(cur_id, parent_id + 1, edge_id + 2);
                    edge_id += 3;
                }
                _ => (),
            }
        }
        Self { graph, bb }
    }

    pub fn to_insts(self) -> Vec<Instruction> {
        let mut insts = vec![];
        for (id, bb) in self.bb {
            insts.append(&mut bb.into());
        }
        insts
    }
}
