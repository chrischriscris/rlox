use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OpConstant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn = 255,
}

impl TryFrom<u8> for OpCode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::OpConstant),
            1 => Ok(OpCode::OpAdd),
            2 => Ok(OpCode::OpSubtract),
            3 => Ok(OpCode::OpMultiply),
            4 => Ok(OpCode::OpDivide),
            5 => Ok(OpCode::OpNegate),
            255 => Ok(OpCode::OpReturn),
            _ => Err(value),
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u32>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_op_code(&mut self, op_code: OpCode, line: u32) {
        self.write(op_code as u8, line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.len() - 1
    }
}
