use std::io;

pub mod lexer;
pub mod repl;
pub mod token;

fn main() -> io::Result<()> {
    repl::start()
}
