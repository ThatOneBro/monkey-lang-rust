use std::io::{self, BufRead, Write};

use crate::lexer::Lexer;
use crate::token::Token;

pub fn start() -> io::Result<()> {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        print!(">> ");
        io::stdout().flush().ok();

        // Clear buffer before reading next line
        line.clear();

        let n = handle.read_line(&mut line)?;

        if n == 0 {
            // EOF (Ctrl+D)
            println!();
            break;
        }

        if line.trim() == ".quit" || line.trim() == ".exit" {
            break;
        }

        let mut lexer = Lexer::new(&line);
        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);

            if tok == Token::Eof {
                break;
            }
        }
    }

    Ok(())
}
