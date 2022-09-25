use std::collections::HashMap;
use std::io::{Read, Write};
use std::{env, fs};

#[derive(Debug)]
enum OpCode {
    SHR,
    SHL,
    ADD,
    SUB,
    GETCHAR,
    PUTCHAR,
    LB,
    RB,
}

#[derive(Debug)]
struct Code {
    instrs: Vec<OpCode>,
    tables: HashMap<usize, usize>,
}

impl Code {
    fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instrs = Vec::new();
        let mut jstack = Vec::new();
        let mut tables = HashMap::new();
        for v in data {
            match v {
                b'>' => instrs.push(OpCode::SHR),
                b'<' => instrs.push(OpCode::SHL),
                b'+' => instrs.push(OpCode::ADD),
                b'-' => instrs.push(OpCode::SUB),
                b'.' => instrs.push(OpCode::GETCHAR),
                b',' => instrs.push(OpCode::PUTCHAR),
                b'[' => {
                    jstack.push(instrs.len());
                    instrs.push(OpCode::LB);
                }
                b']' => {
                    let j = jstack.pop().ok_or("pop from empty stack")?;
                    tables.insert(instrs.len(), j);
                    tables.insert(j, instrs.len());
                    instrs.push(OpCode::RB);
                }
                _ => {}
            };
        }
        Ok(Code { instrs, tables })
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stacks = vec![0u8; 1];
        let (mut pc, mut sp) = (0, 0);
        loop {
            if pc >= self.instrs.len() {
                break;
            }
            match self.instrs[pc] {
                OpCode::SHR => {
                    if sp == stacks.len() - 1 {
                        stacks.push(0);
                    }
                    sp += 1;
                }
                OpCode::SHL => {
                    if sp == 0 {
                        break;
                    }
                    sp -= 1;
                }
                OpCode::ADD => {
                    let (a, _) = (stacks[sp] as u8).overflowing_add(1);
                    stacks[sp] = a;
                }
                OpCode::SUB => {
                    let (a, _) = (stacks[sp] as u8).overflowing_sub(1);
                    stacks[sp] = a;
                }
                OpCode::GETCHAR => {
                    std::io::stdout().write_all(&stacks[sp..=sp])?;
                }
                OpCode::PUTCHAR => {
                    std::io::stdin().read_exact(&mut stacks[sp..=sp])?;
                }
                OpCode::LB => {
                    if stacks[sp] == 0x00 {
                        pc = self.tables[&pc];
                    }
                }
                OpCode::RB => {
                    if stacks[sp] != 0x00 {
                        pc = self.tables[&pc];
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let data = fs::read(&args[1])?;
    let code = Code::from(data)?;
    code.run()?;
    Ok(())
}
