use crate::chunk::{Chunk, OpCode};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM<'a> {
    pub chunk: Option<&'a Chunk>,
    pub ip: usize,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        VM { chunk: None, ip: 0 }
    }

    pub fn interpret(&mut self, chunk: &'a Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;

        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {

            let instruction = match self.read_byte() {
                Some(value) => OpCode::try_from(value),
                None => return InterpretResult::Ok,
            };

            if self.chunk.is_none() {
                return InterpretResult::Ok;
            }

            let instruction = OpCode::try_from(self.read_byte());
            if let Err(_) = instruction {
                return InterpretResult::CompileError;
            }

            return match instruction.unwrap() {
                OpCode::OpReturn => InterpretResult::Ok,
                _ => InterpretResult::CompileError,
            };
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        let mut instruction = None;
        if let Some(chunk) = self.chunk {
            instruction = Some(chunk.code[self.ip]);
            self.ip += 1;
        }

        instruction
    }
}
