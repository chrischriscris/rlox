use std::{env, fs, io, process};

use vm::VM;

mod chunk;
mod debug;
mod value;
mod vm;
mod compiler;
mod scanner;
mod token;

fn repl() {
    loop {
        print!("> ");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let mut vm = VM::new();
        vm.interpret(line);
    }
}

fn run_file(file: &str) -> Result<(), ()> {
    let source = fs::read_to_string(file).map_err(|_| {
        eprintln!("Could not read file '{}'", file);
        process::exit(74);
    }).unwrap();

    let mut vm = VM::new();
    vm.interpret(&source);

    Ok(())
}

fn main() -> Result<(), ()> {
    // Get arguments
    let args: Vec<String> = env::args().collect();
    let res = match args.len() {
        1 => {
            repl();
        }
        2 => {
            run_file(&args[1])?;
        }
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64);
        }
    };


    Ok(())
}
