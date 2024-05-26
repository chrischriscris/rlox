use crate::{chunk::{Chunk, OpCode}, value::print_value};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset-1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset])
    }

    let instruction = &chunk.code[offset];
    let opcode = OpCode::try_from(*instruction);

    if let Err(value) = opcode {
        println!("Unknown opcode {:?}", value);
        return offset + 1;
    }

    return match opcode.unwrap() {
        OpCode::OpConstant => constant_instruction("OpConstant", chunk, offset),
        OpCode::OpAdd => simple_instruction("OpAdd", offset),
        OpCode::OpSubtract => simple_instruction("OpSubtract", offset),
        OpCode::OpMultiply => simple_instruction("OpMultiply", offset),
        OpCode::OpDivide => simple_instruction("OpDivide", offset),
        OpCode::OpNegate => simple_instruction("OpNegate", offset),
        OpCode::OpReturn => simple_instruction("OpReturn", offset),

        // To allow adding new OpCodes without inmediately having to add them
        // here
        #[allow(unreachable_patterns)]
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
