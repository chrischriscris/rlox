use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

mod chunk;
mod debug;
mod value;

fn main() -> Result<(), ()> {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);

    chunk.write_op_code(OpCode::OpConstant);
    chunk.write(constant.try_into().unwrap());
    chunk.write_op_code(OpCode::OpReturn);

    disassemble_chunk(&chunk, "test chunk");


    Ok(())
}
