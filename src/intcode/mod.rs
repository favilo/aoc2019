#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

#[derive(Debug)]
pub enum Opcode {
    ADD(usize, usize, usize),
    MULT(usize, usize, usize),
    HALT,
}

impl Opcode {
    #[inline]
    fn size(&self) -> usize {
        #[cfg(feature = "profiler")]
        profile_scope!("get_opcode");

        match self {
            Self::ADD(_, _, _) => 4,
            Self::MULT(_, _, _) => 4,
            Self::HALT => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    ip: usize,
    memory: Vec<usize>,
}

impl Program {
    pub fn new(input: &Vec<usize>) -> Self {
        #[cfg(feature = "profiler")]
        profile_scope!("new");
        Self {
            ip: 0,
            memory: input.to_vec(),
        }
    }

    pub fn interpret(&mut self) -> usize {
        #[cfg(feature = "profiler")]
        profile_scope!("interpret");
        loop {
            match self.get_opcode() {
                Opcode::ADD(s1, s2, d) => {
                    // log::trace!("{} + {} -> {}", self.memory[s1], self.memory[s2], d);
                    self.memory[d] = self.memory[s1] + self.memory[s2];
                }
                Opcode::MULT(s1, s2, d) => {
                    // log::trace!("{} * {} -> {}", self.memory[s1], self.memory[s2], d);
                    self.memory[d] = self.memory[s1] * self.memory[s2];
                }
                Opcode::HALT => {
                    break;
                }
            }
            self.advance();
        }
        self.memory[0]
    }

    #[inline]
    pub fn set_mem(&mut self, ip: usize, value: usize) {
        #[cfg(feature = "profiler")]
        profile_scope!("set_mem");
        self.memory[ip] = value;
    }

    fn get_bytecode(&self) -> usize {
        self.memory[self.ip]
    }

    #[inline]
    fn get_opcode(&self) -> Opcode {
        #[cfg(feature = "profiler")]
        profile_scope!("get_opcode");
        let ip = self.ip;
        if ip >= self.memory.len() {
            panic!("Didn't halt before end of input");
        }
        match self.memory[ip] {
            1 => Opcode::ADD(
                self.memory[ip + 1],
                self.memory[ip + 2],
                self.memory[ip + 3],
            ),
            2 => Opcode::MULT(
                self.memory[ip + 1],
                self.memory[ip + 2],
                self.memory[ip + 3],
            ),
            99 => Opcode::HALT,
            _ => panic!("Bad Opcode"),
        }
    }

    #[inline]
    fn advance(&mut self) {
        #[cfg(feature = "profiler")]
        profile_scope!("advance");
        self.ip += Self::opcode_size(self.memory[self.ip])
    }

    #[inline]
    fn opcode_size(_opcode: usize) -> usize {
        4
    }
}
