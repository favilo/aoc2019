#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use std::{fmt::Debug, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    ADD(isize, isize, isize),
    MULT(isize, isize, isize),
    INPUT(isize),
    OUTPUT(isize),
    JNZ(isize, isize),
    JZ(isize, isize),
    LESS(isize, isize, isize),
    EQ(isize, isize, isize),
    HALT,
    ERROR(isize),
}

#[derive(Debug, Copy, Clone)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl From<usize> for ParameterMode {
    fn from(i: usize) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("Not implemented yet"),
        }
    }
}

type ParameterModes = (ParameterMode, ParameterMode, ParameterMode);

pub struct Operation {
    modes: ParameterModes,
    opcode: Opcode,
}

impl Operation {
    #[inline]
    fn decode(p: &Program, ip: usize) -> Self {
        let word = p.get(ip as isize) as usize;
        let modes = (
            (word / 100 % 10).into(),
            (word / 1000 % 10).into(),
            (word / 10000).into(),
        );
        let opcode = match word % 100 {
            1 => Opcode::ADD(p.get_offset(1), p.get_offset(2), p.get_offset(3)),
            2 => Opcode::MULT(p.get_offset(1), p.get_offset(2), p.get_offset(3)),
            3 => Opcode::INPUT(p.get_offset(1)),
            4 => Opcode::OUTPUT(p.get_offset(1)),
            5 => Opcode::JNZ(p.get_offset(1), p.get_offset(2)),
            6 => Opcode::JZ(p.get_offset(1), p.get_offset(2)),
            7 => Opcode::LESS(p.get_offset(1), p.get_offset(2), p.get_offset(3)),
            8 => Opcode::EQ(p.get_offset(1), p.get_offset(2), p.get_offset(3)),
            99 => Opcode::HALT,
            op => Opcode::ERROR(op as isize),
        };
        Self { modes, opcode }
    }

    #[inline]
    #[allow(dead_code)]
    fn size(&self) -> usize {
        #[cfg(feature = "profiler")]
        profile_scope!("size");

        match self.opcode {
            Opcode::ADD(_, _, _) => 4,
            Opcode::MULT(_, _, _) => 4,
            Opcode::INPUT(_) => 2,
            Opcode::OUTPUT(_) => 2,
            Opcode::JNZ(_, _) => 3,
            Opcode::JZ(_, _) => 3,
            Opcode::LESS(_, _, _) => 4,
            Opcode::EQ(_, _, _) => 4,
            Opcode::HALT => 1,
            Opcode::ERROR(_) => 1,
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
    memory: Vec<isize>,
    status: RunningStatus,
}

impl Program {
    pub fn new(input: &Vec<isize>) -> Self {
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
    pub fn load_input(&mut self, noun: isize, verb: isize) {
        self.set(1, noun);
        self.set(2, verb);
    }

    #[allow(dead_code)]
    pub fn interpret_iter<T>(self, input: T) -> ProgramIter<T>
    where
        T: Iterator<Item = isize>,
    {
        #[cfg(feature = "profiler")]
        profile_scope!("interpret");

        ProgramIter { p: self, input }
    }

    pub fn interpret_input(&mut self, mut input: impl Iterator<Item = isize>) -> Vec<isize> {
        let mut vec = Vec::new();
        loop {
            let output = self.interpret_to_output(&mut input);
            if output.is_none() {
                break;
            }
            vec.push(output.unwrap());
        }

        vec
    }

    pub fn interpret(&mut self) -> isize {
        self.interpret_input(vec![].into_iter());
        self.get(0)
    }

    #[inline]
    fn set_mode(&mut self, mode: ParameterMode, ip: isize, value: isize) {
        #[cfg(feature = "profiler")]
        profile_scope!("set");
        match mode {
            ParameterMode::Immediate => self.set(ip, value),
            ParameterMode::Position => self.set(ip, value),
        };
    }

    #[inline]
    fn set(&mut self, ip: isize, value: isize) {
        #[cfg(feature = "profiler")]
        profile_scope!("set");
        self.memory[ip as usize] = value;
    }

    #[inline]
    #[allow(dead_code)]
    fn set_indirect(&mut self, ip: isize, value: isize) {
        let index = self.memory[self.get(ip) as usize] as usize;
        self.memory[index] = value;
    }

    #[inline]
    fn get(&self, ip: isize) -> isize {
        self.memory[ip as usize]
    }

    fn get_mode(&self, mode: ParameterMode, ip: isize) -> isize {
        match mode {
            ParameterMode::Immediate => ip,
            ParameterMode::Position => self.get(ip),
        }
    }

    #[inline]
    fn get_offset(&self, offset: isize) -> isize {
        self.memory[(self.ip as isize + offset) as usize]
    }

    #[inline]
    fn advance(&mut self, op: Operation) {
        #[cfg(feature = "profiler")]
        profile_scope!("advance");
        self.cycles += 1;
        self.ip += op.size();
    }

    pub fn interpret_to_output<T>(&mut self, input: &mut T) -> Option<T::Item>
    where
        T: Iterator<Item = isize>,
    {
        loop {
            let mut jumped = false;
            if self.ip >= self.memory.len() {
                panic!("Didn't halt before end of input");
            }

            let op = Operation::decode(self, self.ip);
            match op.opcode {
                Opcode::ADD(s1, s2, d) => {
                    // println!(
                    //     "ADD {:?}:{}, {:?}:{} -> {:?}:{}",
                    //     op.modes.0, s1, op.modes.1, s2, op.modes.2, d
                    // );
                    let a = self.get_mode(op.modes.0, s1);
                    let b = self.get_mode(op.modes.1, s2);
                    self.set_mode(op.modes.2, d, a + b);
                }
                Opcode::MULT(s1, s2, d) => {
                    // println!(
                    //     "MULT {:?}:{}, {:?}:{} -> {:?}:{}",
                    //     op.modes.0, s1, op.modes.1, s2, op.modes.2, d
                    // );
                    let a = self.get_mode(op.modes.0, s1);
                    let b = self.get_mode(op.modes.1, s2);
                    self.set_mode(op.modes.2, d, a * b);
                }
                Opcode::INPUT(i1) => {
                    // println!("INPUT {:?}:{}", op.modes.0, i1);
                    self.set(i1, input.next().expect("Not enough input"));
                }
                Opcode::OUTPUT(o1) => {
                    // println!("OUTPUT {:?}:{}", op.modes.0, o1);
                    self.advance(op);
                    return Some(self.get(o1));
                }
                Opcode::JNZ(test, dest) => {
                    // println!("JNZ {:?}:{} {:?}:{}", op.modes.0, test, op.modes.1, dest);
                    if self.get_mode(op.modes.0, test) != 0 {
                        self.ip = self.get_mode(op.modes.1, dest) as usize;
                        jumped = true;
                    }
                }
                Opcode::JZ(test, dest) => {
                    if self.get_mode(op.modes.0, test) == 0 {
                        self.ip = self.get_mode(op.modes.1, dest) as usize;
                        jumped = true;
                    }
                }
                Opcode::LESS(s1, s2, dest) => {
                    let a = self.get_mode(op.modes.0, s1);
                    let b = self.get_mode(op.modes.1, s2);
                    self.set_mode(op.modes.2, dest, if a < b { 1 } else { 0 });
                }
                Opcode::EQ(s1, s2, dest) => {
                    let a = self.get_mode(op.modes.0, s1);
                    let b = self.get_mode(op.modes.1, s2);
                    self.set_mode(op.modes.2, dest, if a == b { 1 } else { 0 });
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
            if !jumped {
                self.advance(op);
            }
        }
        None
    }
}

impl FromStr for Program {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Program, Self::Err> {
        let v = input
            .lines()
            .map(|s| s.split(',').map(isize::from_str))
            .flatten()
            .collect::<Result<Vec<isize>, Self::Err>>();
        Ok(Program::new(&v.unwrap()))
    }
}

#[derive(Clone)]
pub struct ProgramIter<A> {
    p: Program,
    input: A,
}

impl<A> Iterator for ProgramIter<A>
where
    A: Iterator<Item = isize>,
{
    type Item = isize;
    fn next(&mut self) -> Option<isize> {
        self.p.interpret_to_output(&mut self.input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(test: &str, input: Vec<isize>) -> (Vec<isize>, Vec<isize>) {
        let mut p: Program = test.parse().unwrap();
        let (_, output) = p.interpret_input(input.into_iter());
        (p.memory, output)
    }

    #[test]
    fn test_simple() {
        assert_eq!(run("1,0,0,0,99", vec![]).0, vec![2, 0, 0, 0, 99]);
        assert_eq!(run("2,3,0,3,99", vec![]).0, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_modes() {
        assert_eq!(run("1101,100,-1,4,0", vec![]).0, vec![1101, 100, -1, 4, 99]);
        assert_eq!(run("1002,4,3,4,33", vec![]).0, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_io() {
        let (mem, output) = run("3,9,8,9,10,9,4,9,99,-1,8", vec![9]);
        assert_eq!(mem, vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]);
        assert_eq!(output, vec![0]);
        let (mem, output) = run("3,9,8,9,10,9,4,9,99,-1,8", vec![8]);
        assert_eq!(mem, vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8]);
        assert_eq!(output, vec![1]);
        let (mem, output) = run("3,9,7,9,10,9,4,9,99,-1,8", vec![8]);
        assert_eq!(mem, vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]);
        assert_eq!(output, vec![0]);
        let (mem, output) = run("3,9,7,9,10,9,4,9,99,-1,8", vec![1]);
        assert_eq!(mem, vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8]);
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn test_jumps() {
        let (mem, output) = run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![9]);
        assert_eq!(mem, vec![3, 3, 1105, 9, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(output, vec![1]);
        let (mem, output) = run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![0]);
        assert_eq!(
            mem,
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9]
        );
        assert_eq!(output, vec![0]);
    }
}
