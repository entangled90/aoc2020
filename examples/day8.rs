use aoc2020::aoc::{load_data, Res};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::io::BufRead;
use Instruction::*;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i64),
    Jump(i64),
    Nop(i64), // do not use value, just needed for flipping it.
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: &Vec<Instruction>) -> Program {
        Program {
            instructions: instructions.clone(),
        }
    }
}

#[derive(Debug)]
struct ProgramExecution {
    steps: Vec<usize>,
    visited: Vec<bool>,
    pointer: usize,
    acc: i64,
}

impl ProgramExecution {
    pub fn new(p: &Program) -> ProgramExecution {
        ProgramExecution {
            steps: Vec::new(),
            visited: vec![false; p.instructions.len()],
            pointer: 0,
            acc: 0,
        }
    }
}

// none is not terminating
fn interpret(p: &Program, exec: &mut ProgramExecution) -> Res<Option<i64>> {
    let visited = exec.visited.get_mut(exec.pointer);
    exec.steps.push(exec.pointer);
    match visited {
        Some(false) => {
            let visited = visited.unwrap();
            *visited = true;
            match p.instructions[exec.pointer] {
                Acc(delta) => {
                    exec.pointer += 1;
                    exec.acc += delta;
                }
                Jump(to) => exec.pointer = (exec.pointer as i64 + to) as usize,
                Nop(_) => exec.pointer += 1,
            }
            interpret(p, exec)
        }
        // terminate, because of cycle
        Some(true) => Ok(None),
        // out of bounds, correct termination
        None => Ok(Some(exec.acc)),
    }
}

fn read_instructions(reader: std::io::BufReader<std::fs::File>) -> Res<Vec<Instruction>> {
    let mut instructions = Vec::with_capacity(64);
    for line in reader.lines() {
        let l = line?;
        let splitted: Vec<_> = l.split(" ").collect();
        match splitted.as_slice() {
            [cmd, n] => {
                let n_parsed = n.parse::<i64>()?;
                let instruction = match cmd {
                    &"acc" => Instruction::Acc(n_parsed),
                    &"jmp" => Instruction::Jump(n_parsed),
                    &"nop" => Instruction::Nop(n_parsed),
                    other => return Err(format!("invalid line {}", other).into()),
                };
                instructions.push(instruction);
            }
            other => return Err(format!("invalid line {:?}, splitted {:?}", l, other).into()),
        }
    }
    Ok(instructions)
}

fn execute(instructions: &Vec<Instruction>) -> Res<(Option<i64>, Program, ProgramExecution)> {
    let mut program = Program::new(&instructions);
    let mut execution = ProgramExecution::new(&program);
    let res = interpret(&mut program, &mut execution)?;
    Ok((res, program, execution))
}

fn main() -> Res<()> {
    let data = load_data("examples/data/day8.txt")?;
    let instructions = read_instructions(data)?;
    let (result, _, execution) = execute(&instructions)?;
    println!("Result: {:?}, {:?}", result, execution);
    let mut instructions_modified = instructions.clone();
    let steps_len = execution.steps.len();
    let mut result = None;
    for i in 1..steps_len {
        let idx = execution.steps[steps_len - i];
        let modified = instructions_modified.get_mut(idx).unwrap();
        match modified {
            Instruction::Jump(delta) => *modified = Instruction::Nop(*delta),
            Instruction::Nop(delta) => *modified = Instruction::Jump(*delta),
            _ => continue,
        }
        let (res, _, exec) = execute(&instructions_modified)?;
        if let Some(valid) = res {
            println!("Modified idx {:?}", idx);
            println!("Modified steps: {:?}", exec.steps);
            result = res;
            break;
        }
    }
    println!("Result is {:?}", result);
    Ok(())
}
