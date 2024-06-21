use crate::{
    chunk::{Chunk, OpCode},
    compiler,
    debug::disassemble_instruction,
    value::{print_value, Value},
};

#[macro_export]
macro_rules! binary_op {
    ($vm:ident, $op:tt) => {{
        let b = $vm.pop();
        let a = $vm.pop();
        $vm.push(a $op b);
    }};
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

const STACK_MAX: usize = 256;

pub struct VM {
    pub chunk: Option<Chunk>,
    pub ip: usize,
    pub stack: [Value; STACK_MAX],
    pub stack_top: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: None,
            ip: 0,
            stack: [0.0; STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        match compiler::compile(source) {
            Ok(chunk) => {
                self.chunk = Some(chunk);
                self.ip = 0;
                self.stack_top = 0;

                let result = self.run();

                result
            }
            Err(_) => InterpretResult::CompileError,
        }
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                self.stack[..self.stack_top]
                    .iter()
                    .for_each(|&slot| print!("[{}]", slot));
                println!();

                disassemble_instruction(self.chunk.unwrap(), self.ip);
            }

            let instruction = match self.read_byte() {
                Some(value) => OpCode::try_from(value),
                None => return InterpretResult::Ok,
            };

            match instruction.unwrap() {
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                OpCode::OpAdd => binary_op!(self, +),
                OpCode::OpSubtract => binary_op!(self, -),
                OpCode::OpMultiply => binary_op!(self, *),
                OpCode::OpDivide => binary_op!(self, /),
                OpCode::OpNegate => {
                    let val = -self.pop();
                    self.push(val);
                }

                OpCode::OpReturn => {
                    print_value(self.pop());
                    println!();
                    break InterpretResult::Ok;
                }
                _ => break InterpretResult::CompileError,
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

    // Operations over stack

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;

        self.stack[self.stack_top]
    }
}
