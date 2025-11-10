#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers, literals
    Identifier,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

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

pub fn check_identifier_or_keyword(candidate: &str) -> TokenType {
    match candidate {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        _ => TokenType::Identifier,
    }
}
