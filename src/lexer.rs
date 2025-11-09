use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn _peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn next_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn next_token(&mut self) -> Token {
        match self.next_char() {
            Some('=') => Token::from_char(TokenType::Assign, '='),
            Some('+') => Token::from_char(TokenType::Plus, '+'),
            Some('(') => Token::from_char(TokenType::LParen, '('),
            Some(')') => Token::from_char(TokenType::RParen, ')'),
            Some('{') => Token::from_char(TokenType::LBrace, '{'),
            Some('}') => Token::from_char(TokenType::RBrace, '}'),
            Some(',') => Token::from_char(TokenType::Comma, ','),
            Some(';') => Token::from_char(TokenType::Semicolon, ';'),
            Some(ch) => Token::from_char(TokenType::Illegal, ch),
            None => Token::new(TokenType::Eof, String::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                tok.token_type, *expected_type,
                "tests[{}] - token type wrong. expected={:?}, got={:?}",
                i, expected_type, tok.token_type
            );

            assert_eq!(
                tok.literal, *expected_literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expected_literal, tok.literal
            );
        }
    }
}
