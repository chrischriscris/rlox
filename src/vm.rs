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

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = match self.read_byte() {
                Some(value) => OpCode::try_from(value),
                None => return InterpretResult::Ok,
            };

            return match instruction.unwrap() {
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                    println!("{}", constant);

                    break InterpretResult::Ok
                }
                OpCode::OpReturn => InterpretResult::Ok,
                _ => InterpretResult::CompileError,
            };
        }
    }

    // These functions are used when there exists a chunk, so we can safely unwrap

    /// Read a byte from the current chunk's code and increment the
    /// instruction pointer when successful
    ///
    /// # Returns
    /// - `Some(u8)` - The byte read from the code
    /// - `None` - If the instruction pointer is out of bounds
    fn read_byte(&mut self) -> Option<u8> {
        let instruction = self.chunk.unwrap().code.get(self.ip).copied();
        if instruction.is_some() {
            self.ip += 1;
        }

        instruction
    }

    /// Read a constant from the current chunk's constants
    ///
    /// # Returns
    /// - `f64` - The constant value
    /// - `0.0` - If the chunk is `None`
    fn read_constant(&mut self) -> f64 {
        let index = self.read_byte().unwrap();

        if let Some(chunk) = self.chunk {
            chunk.constants.values[index as usize]
        } else {
            0.0
        }
    }
}
