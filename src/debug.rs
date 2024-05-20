use crate::{chunk::{Chunk, OpCode}, value::print_value};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let instruction = &chunk.code[offset];
    let opcode = OpCode::try_from(*instruction);

    if let Err(value) = opcode {
        println!("Unknown opcode {:?}", value);
        return offset + 1;
    }

    return match opcode.unwrap() {
        OpCode::OpConstant => constant_instruction("OpConstant", chunk, offset),
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

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:<16} {:4} '", name, constant);
    print_value(chunk.constants.values[constant as usize]);
    println!("'");

    offset + 2
}
