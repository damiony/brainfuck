use std::{
    fs,
    io::{Read, Write},
};

enum IR {
    SHR(usize),
    SHL(usize),
    ADD(usize),
    SUB(usize),
    GETCHAR,
    PUTCHAR,
    JIZ(usize),
    JNZ(usize),
}

struct Code {
    instrs: Vec<IR>,
}

impl Code {
    fn from(data: Vec<u8>) -> Result<Code, Box<dyn std::error::Error>> {
        let mut instrs = Vec::new();
        let mut jstack = Vec::new();
        for v in data {
            match v {
                b'>' => match instrs.last_mut() {
                    Some(IR::SHR(i)) => *i += 1,
                    _ => instrs.push(IR::SHR(1)),
                },
                b'<' => match instrs.last_mut() {
                    Some(IR::SHL(i)) => *i += 1,
                    _ => instrs.push(IR::SHL(1)),
                },
                b'+' => match instrs.last_mut() {
                    Some(IR::ADD(i)) => *i += 1,
                    _ => instrs.push(IR::ADD(1)),
                },
                b'-' => match instrs.last_mut() {
                    Some(IR::SUB(i)) => *i += 1,
                    _ => instrs.push(IR::SUB(1)),
                },
                b'.' => instrs.push(IR::GETCHAR),
                b',' => instrs.push(IR::PUTCHAR),
                b'[' => {
                    instrs.push(IR::JIZ(0));
                    jstack.push(instrs.len() - 1);
                }
                b']' => {
                    let j = jstack.pop().ok_or("pop from empty stack")?;
                    instrs.push(IR::JNZ(j));
                    instrs[j] = IR::JIZ(instrs.len() - 1);
                }
                _ => {}
            }
        }
        Ok(Code { instrs })
    }
}

struct Interpreter {}

impl Interpreter {
    fn run(code: Code) -> Result<(), Box<dyn std::error::Error>> {
        let mut stack = vec![0u8; 1];
        let (mut pc, mut sp) = (0usize, 0usize);
        loop {
            if pc == code.instrs.len() {
                break;
            }
            match code.instrs[pc] {
                IR::SHR(v) => {
                    sp += v;
                    if sp >= stack.len() {
                        let offset = sp - stack.len() + 1;
                        for _ in 0..offset {
                            stack.push(0);
                        }
                    }
                }
                IR::SHL(v) => {
                    if sp < v {
                        break;
                    }
                    sp -= v;
                }
                IR::ADD(v) => {
                    let (ans, _) = (stack[sp] as usize).overflowing_add(v);
                    stack[sp] = ans as u8;
                }
                IR::SUB(v) => {
                    let (ans, _) = (stack[sp] as usize).overflowing_sub(v);
                    stack[sp] = ans as u8;
                }
                IR::GETCHAR => {
                    std::io::stdout().write_all(&mut stack[sp..=sp])?;
                }
                IR::PUTCHAR => {
                    std::io::stdin().read_exact(&mut stack[sp..=sp])?;
                }
                IR::JIZ(v) => {
                    if stack[sp] == 0x00 {
                        pc = v;
                    }
                }
                IR::JNZ(v) => {
                    if stack[sp] != 0x00 {
                        pc = v;
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).ok_or("no file name provided")?;
    let data = fs::read(filename)?;
    let code = Code::from(data)?;
    Interpreter::run(code)?;
    Ok(())
}
