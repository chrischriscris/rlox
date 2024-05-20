use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

mod chunk;
mod debug;
mod value;

fn main() -> Result<(), ()> {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn);

    disassemble_chunk(&chunk, "test chunk");


    Ok(())
}
