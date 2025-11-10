use crate::token::{Token, get_identifier_or_keyword};

pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    fn next_char(&mut self) -> Option<u8> {
        if self.pos < self.input.len() {
            let ch = self.input[self.pos];
            self.pos += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                b' ' | b'\t' | b'\n' | b'\r' => {
                    self.next_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn read_identifier(&mut self) -> &'a str {
        let start = self.pos - 1;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == b'_' {
                self.next_char();
            } else {
                break;
            }
        }
        std::str::from_utf8(&self.input[start..self.pos]).unwrap()
    }

    fn read_number(&mut self) -> i32 {
        let start = self.pos - 1;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.next_char();
            } else {
                break;
            }
        }
        let num_str = std::str::from_utf8(&self.input[start..self.pos]).unwrap();
        num_str.parse().unwrap()
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        match self.next_char() {
            Some(b'=') => {
                if let Some(b'=') = self.peek() {
                    self.next_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some(b'+') => Token::Plus,
            Some(b'-') => Token::Minus,
            Some(b'!') => {
                if let Some(b'=') = self.peek() {
                    self.next_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            Some(b'*') => Token::Asterisk,
            Some(b'/') => Token::Slash,
            Some(b'>') => Token::Gt,
            Some(b'<') => Token::Lt,
            Some(b'(') => Token::LParen,
            Some(b')') => Token::RParen,
            Some(b'{') => Token::LBrace,
            Some(b'}') => Token::RBrace,
            Some(b',') => Token::Comma,
            Some(b';') => Token::Semicolon,
            Some(ch) if ch.is_ascii_alphabetic() || ch == b'_' => {
                let literal = self.read_identifier();
                get_identifier_or_keyword(&literal)
            }
            Some(ch) if ch.is_ascii_digit() => {
                let literal = self.read_number();
                Token::Int(literal)
            }
            Some(_ch) => Token::Illegal,
            None => Token::Eof,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

        let tests = vec![
            Token::Let,
            Token::Identifier("five"),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten"),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add"),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Identifier("x"),
            Token::Comma,
            Token::Identifier("y"),
            Token::RParen,
            Token::LBrace,
            Token::Identifier("x"),
            Token::Plus,
            Token::Identifier("y"),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result"),
            Token::Assign,
            Token::Identifier("add"),
            Token::LParen,
            Token::Identifier("five"),
            Token::Comma,
            Token::Identifier("ten"),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for (i, expected_token) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                tok, *expected_token,
                "tests[{}] - token wrong. expected={:?}, got={:?}",
                i, expected_token, tok,
            );
        }
    }
}
