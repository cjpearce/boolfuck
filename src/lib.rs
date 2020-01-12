pub fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let program = Program::compile(code);
    let mut runtime = Runtime::new(program, input);
    runtime.run();
    runtime.output
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Read,
    FlipBit,
    JmpF(usize),
    JmpT(usize),
    Write,
    IncP,
    DecP,
}

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn compile(code: &str) -> Self {
        let mut jumps = Vec::new();
        let mut instructions = Vec::with_capacity(code.len());

        for (i, c) in code.chars().enumerate() {
            match c {
                '>' => instructions.push(Instruction::IncP),
                '<' => instructions.push(Instruction::DecP),
                '+' => instructions.push(Instruction::FlipBit),
                ';' => instructions.push(Instruction::Write),
                ',' => instructions.push(Instruction::Read),
                '[' => {
                    jumps.push(i);
                    instructions.push(Instruction::JmpF(0))
                }
                ']' => {
                    let jmp_position = jumps.pop().unwrap();
                    *instructions.get_mut(jmp_position).unwrap() = Instruction::JmpF(i);
                    instructions.push(Instruction::JmpT(jmp_position));
                }
                _ => {
                    // Ignored
                }
            }
        }

        Program { instructions }
    }
}

#[derive(Debug)]
pub struct Tape {
    data: Vec<u8>,
    mask: u8,
    position: usize
}

impl Tape {
    pub fn new() -> Self {
        Self{
            data: vec![0],
            mask: 1u8,
            position: 0
        }
    }

    pub fn write(&mut self, value: bool) {
        assert!(self.position < self.data.len());
        if value {
            self.data[self.position] ^= self.mask;
        } else {
            self.data[self.position] &= !self.mask;
        }
    }

    pub fn inc_p(&mut self) {
        if self.mask == 0b10000000 {
            self.mask = 0b00000001;

            if self.position == 1 {
                self.position = 0;
            } else if self.position % 2 == 0 {
                self.position += 2;
            } else {
                self.position -= 2;
            }
        } else {
            self.mask = self.mask << 1;
        }

        if self.data.len() <= self.position {
            self.data.push(0);
            self.data.push(0);
        }
    }

    pub fn dec_p(&mut self) {
        if self.mask == 0b00000001 {
            self.mask = 0b10000000;

            if self.position == 0 {
                self.position = 1;
            } else if self.position % 2 == 1 {
                self.position += 2;
            } else {
                self.position -= 2;
            }
        } else {
            self.mask = self.mask >> 1;
        }

        if self.data.len() <= self.position {
            self.data.push(0);
            self.data.push(0);
        }
    }

    pub fn flip(&mut self) {
        self.data[self.position] = self.data[self.position] ^ self.mask
    }

    pub fn read(&self) -> bool {
        self.data[self.position] & self.mask == self.mask
    }
}

#[derive(Debug)]
pub struct Runtime {
    tape: Tape,
    ip: usize,
    input_mask: u8,
    input_position: usize,
    output_mask: u8,
    output: Vec<u8>,
    program: Program,
    input: Vec<u8>,
}

impl Runtime {
    pub fn new(program: Program, input: Vec<u8>) -> Self {
        Runtime {
            tape: Tape::new(),
            ip: 0,
            input_mask: 1,
            input_position: 0,
            output_mask: 0b10000000,
            output: vec![],
            program,
            input,
        }
    }

    pub fn run(&mut self) {
        while let Some(_) = self.next() {}
    }
}

impl Iterator for Runtime {
    type Item = Instruction;
    fn next(&mut self) -> Option<Instruction> {
        if self.ip >= self.program.instructions.len() {
            return None;
        }

        let instruction = self.program.instructions[self.ip];
        match instruction {
            Instruction::FlipBit => self.tape.flip(),
            Instruction::JmpF(position) => {
                if !self.tape.read() {
                    self.ip = position
                }
            }
            Instruction::JmpT(position) => {
                if self.tape.read() {
                    self.ip = position
                }
            }
            Instruction::Read => {
                self.tape.write(self.input_mask & self.input[self.input_position] != 0);

                if self.input_mask == 0b10000000 {
                    self.input_mask = 0b00000001;
                    self.input_position += 1;
                } else {
                    self.input_mask = self.input_mask << 1;
                }
            }
            Instruction::Write => {
                if self.output_mask == 0b10000000 {
                    self.output.push(0);
                    self.output_mask = 0b00000001;
                } else {
                    self.output_mask = self.output_mask << 1;
                }

                if let Some(a) = self.output.last_mut() {
                    if self.tape.read() {
                        *a ^= self.output_mask;
                    } else {
                        *a &= !self.output_mask;
                    }
                }
            }
            Instruction::IncP => {
                self.tape.inc_p();
            }
            Instruction::DecP => {
                self.tape.dec_p();
            }
        }
        self.ip += 1;
        Some(instruction)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tape_tests() {
        let mut tape = Tape::new();
        assert_eq!(tape.read(), false);
        tape.flip();
        assert_eq!(tape.read(), true);
        tape.inc_p();
        assert_eq!(tape.read(), false);
        tape.dec_p();
        assert_eq!(tape.read(), true);
        tape.dec_p();
        assert_eq!(tape.read(), false);
        tape.flip();
        assert_eq!(tape.read(), true);
        tape.inc_p();
        assert_eq!(tape.read(), true);
        tape.inc_p();
        assert_eq!(tape.read(), false);
    }

    #[test]
    fn example_test_cases() {
        // Hello World Program taken from the official website
        assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
        // Echo until byte(0) encountered
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
        // Two numbers multiplier
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
    }
}
