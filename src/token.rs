#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers, literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }

    pub fn from_char(token_type: TokenType, ch: char) -> Self {
        Token {
            token_type,
            literal: ch.to_string(),
        }
    }
}
