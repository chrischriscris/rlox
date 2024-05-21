use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;
use vm::VM;

mod chunk;
mod debug;
mod value;
mod vm;

fn main() -> Result<(), ()> {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);

    chunk.write_op_code(OpCode::OpConstant, 42);
    chunk.write(constant.try_into().unwrap(), 42);
    chunk.write_op_code(OpCode::OpReturn, 42);

    disassemble_chunk(&chunk, "test chunk");

    Ok(())
}
