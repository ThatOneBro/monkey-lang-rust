use crate::token::{Token, TokenType, check_identifier_or_keyword};
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

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn next_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while let Some(&ch) = self.peek() {
            if ch.is_alphabetic() || ch == '_' {
                ident.push(self.next_char().unwrap());
            } else {
                break;
            }
        }

        ident
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&ch) = self.peek() {
            if ch.is_ascii_digit() {
                number.push(self.next_char().unwrap());
            } else {
                break;
            }
        }

        number
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.next_char() {
            Some('=') => Token::from_char(TokenType::Assign, '='),
            Some('+') => Token::from_char(TokenType::Plus, '+'),
            Some('-') => Token::from_char(TokenType::Minus, '-'),
            Some('!') => Token::from_char(TokenType::Bang, '!'),
            Some('*') => Token::from_char(TokenType::Asterisk, '*'),
            Some('/') => Token::from_char(TokenType::Slash, '/'),
            Some('>') => Token::from_char(TokenType::Gt, '>'),
            Some('<') => Token::from_char(TokenType::Lt, '<'),
            Some('(') => Token::from_char(TokenType::LParen, '('),
            Some(')') => Token::from_char(TokenType::RParen, ')'),
            Some('{') => Token::from_char(TokenType::LBrace, '{'),
            Some('}') => Token::from_char(TokenType::RBrace, '}'),
            Some(',') => Token::from_char(TokenType::Comma, ','),
            Some(';') => Token::from_char(TokenType::Semicolon, ';'),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let literal = self.read_identifier(ch);
                let token_type = check_identifier_or_keyword(&literal);
                Token::new(token_type, literal)
            }
            Some(ch) if ch.is_ascii_digit() => {
                let literal = self.read_number(ch);
                Token::new(TokenType::Int, literal)
            }
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
        let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;
"#;

        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Identifier, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifier, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifier, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Identifier, "x"),
            (TokenType::Comma, ","),
            (TokenType::Identifier, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Identifier, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Identifier, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifier, "result"),
            (TokenType::Assign, "="),
            (TokenType::Identifier, "add"),
            (TokenType::LParen, "("),
            (TokenType::Identifier, "five"),
            (TokenType::Comma, ","),
            (TokenType::Identifier, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
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
