#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Illegal,
    Eof,

    // Identifiers, literals
    Identifier(&'a str),
    Int(i32),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,

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
    True,
    False,
    If,
    Else,
    Return,
}

pub fn get_identifier_or_keyword<'a>(candidate: &'a str) -> Token<'a> {
    match candidate {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Identifier(candidate),
    }
}
