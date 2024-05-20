use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
        println!();
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let instruction = &chunk.code[offset];

    return match instruction {
        OpCode::OpReturn => simple_instruction("OpReturn", offset),
        _ => {
            println!("Unknown opcode {:?}", instruction);
            offset + 1
        }
    };
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);

    offset + 1
}
