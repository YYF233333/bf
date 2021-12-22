use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::{char, newline},
    combinator::value,
    multi::{many0, many_till},
    sequence::pair,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Add(i32),
    Move(i32),
    Input,
    Output,
    Set,
    Test,
    JmpZero(usize),
    JmpNonZero(usize),
    Clear,
    STOP,
}

fn comment(input: &str) -> IResult<&str, Option<Instruction>> {
    alt((
        value(None, pair(tag("/*"), take_until("*/"))),
        value(None, pair(tag("//"), many_till(take(1 as usize), newline))),
        value(None, pair(tag("#"), many_till(take(1 as usize), newline))),
    ))(input)
}

fn instruction(input: &str) -> IResult<&str, Option<Instruction>> {
    alt((
        value(Some(Instruction::Add(1)), char('+')),
        value(Some(Instruction::Add(-1)), char('-')),
        value(Some(Instruction::Move(-1)), char('<')),
        value(Some(Instruction::Move(1)), char('>')),
        value(Some(Instruction::Output), char('.')),
        value(Some(Instruction::Input), char(',')),
        value(Some(Instruction::Set), char('[')),
        value(Some(Instruction::Test), char(']')),
    ))(input)
}

fn useless(input: &str) -> IResult<&str, Option<Instruction>> {
    value(None, take(1 as usize))(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(alt((instruction, comment, useless)))(input).map(|(remain, insts)| {
        let mut insts: Vec<Instruction> = insts
            .into_iter()
            .filter(|t| t.is_some())
            .map(|t| t.unwrap())
            .collect();
        insts.push(Instruction::STOP);
        (remain, insts)
    })
}
