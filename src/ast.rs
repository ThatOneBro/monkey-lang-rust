use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Identifier(Token<'a>),
    Integer(Token<'a>),
}

impl Expression<'_> {
    pub fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(_) => "IDENT",
            Expression::Integer(_) => "INT",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Let(LetStmt<'a>),
}

#[derive(Debug, Clone)]
pub struct LetStmt<'a> {
    pub name: Token<'a>,
    pub init: Option<Expression<'a>>,
}

impl Statement<'_> {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(_) => "let",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl Program<'_> {
    pub fn token_literal(&self) -> &str {
        self.statements
            .first()
            .map(|s| s.token_literal())
            .unwrap_or("")
    }
}
