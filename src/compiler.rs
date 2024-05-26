use crate::{chunk::Chunk, scanner};

pub fn compile(source: &str) -> Result<Chunk, ()> {
    let mut scanner = scanner::Scanner::new(source);
    let chunk = Chunk::new();

    let mut line = -1;
    loop {
        return Ok(chunk);
    }

    Err(())
}
