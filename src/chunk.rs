use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

impl TryFrom<u8> for OpCode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::OpConstant),
            1 => Ok(OpCode::OpReturn),
            _ => Err(value),
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_op_code(&mut self, op_code: OpCode) {
        self.write(op_code as u8);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.len() - 1
    }
}
