use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

mod chunk;
mod debug;
mod value;

fn main() -> Result<(), ()> {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);

    chunk.write_op_code(OpCode::OpConstant, 42);
    chunk.write(constant.try_into().unwrap(), 42);
    chunk.write_op_code(OpCode::OpReturn, 42);

    disassemble_chunk(&chunk, "test chunk");

    Ok(())
}
