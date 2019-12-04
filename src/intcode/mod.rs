#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::fmt::FromStr;

#[derive(Debug, Clone)]
pub enum Opcode {
    ADD(usize, usize, usize),
    MULT(usize, usize, usize),
    HALT,
    ERROR(usize),
}

impl Opcode {
    #[inline]
    fn decode(p: &Program, ip: usize) -> Self {
        match p.get(ip) {
            1 => Opcode::ADD(p.get_offset(1), p.get_offset(2), p.get_offset(3)),
            2 => Opcode::MULT(p.get_offset(1), p.get_offset(2), p.get_offset(3)),
            99 => Opcode::HALT,
            op => Opcode::ERROR(op),
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn size(&self) -> usize {
        #[cfg(feature = "profiler")]
        profile_scope!("size");

        match self {
            Self::ADD(_, _, _) => 4,
            Self::MULT(_, _, _) => 4,
            Self::HALT => 1,
            Self::ERROR(_) => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RunningStatus {
    Running,
    Halted,
    Killed,
}

#[derive(Debug, Clone)]
pub struct Program {
    ip: usize,
    cycles: usize,
    memory: Vec<usize>,
    status: RunningStatus,
}

impl Program {
    pub fn new(input: &Vec<usize>) -> Self {
        #[cfg(feature = "profiler")]
        profile_scope!("new");
        Self {
            ip: 0,
            cycles: 0,
            memory: input.to_vec(),
            status: RunningStatus::Running,
        }
    }

    #[inline]
    pub fn load_input(&mut self, noun: usize, verb: usize) {
        self.set(1, noun);
        self.set(2, verb);
    }

    pub fn interpret(&mut self) -> usize {
        #[cfg(feature = "profiler")]
        profile_scope!("interpret");
        loop {
            if self.ip >= self.memory.len() {
                panic!("Didn't halt before end of input");
            }

            let opcode = Opcode::decode(self, self.ip);
            match opcode {
                Opcode::ADD(s1, s2, d) => {
                    let a = self.get(s1);
                    let b = self.get(s2);
                    self.set(d, a + b);
                }
                Opcode::MULT(s1, s2, d) => {
                    let a = self.get(s1);
                    let b = self.get(s2);
                    self.set(d, a * b);
                }
                Opcode::HALT => {
                    self.status = RunningStatus::Halted;
                    break;
                }
                Opcode::ERROR(op) => {
                    self.status = RunningStatus::Killed;
                    panic!("Error opcode {}", op);
                }
            }
            self.advance(opcode);
        }
        self.get(0)
    }

    #[inline]
    fn set(&mut self, ip: usize, value: usize) {
        #[cfg(feature = "profiler")]
        profile_scope!("set");
        self.memory[ip] = value;
    }

    #[inline]
    #[allow(dead_code)]
    fn set_indirect(&mut self, offset: i32, value: usize) {
        let index = self.memory[self.get_offset(offset)];
        self.memory[index] = value;
    }

    #[inline]
    fn get(&self, ip: usize) -> usize {
        self.memory[ip]
    }

    #[inline]
    fn get_offset(&self, offset: i32) -> usize {
        self.memory[(self.ip as i32 + offset) as usize]
    }

    #[inline]
    #[allow(dead_code)]
    fn get_indirect(&self, offset: i32) -> usize {
        self.memory[self.get_offset(offset)]
    }

    #[inline]
    fn advance(&mut self, opcode: Opcode) {
        #[cfg(feature = "profiler")]
        profile_scope!("advance");
        self.cycles += 1;
        self.ip += opcode.size();
    }
}

impl FromStr for Program {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Program, Self::Err> {
        Ok(Program::new(
            input.split_terminator(',').map(str::parse::<usize>)
        ))
    }
}
