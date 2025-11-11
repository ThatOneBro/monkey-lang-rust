use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expression<'src> {
    Identifier(Token<'src>),
    Integer(Token<'src>),
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
pub enum Statement<'src> {
    Let(LetStmt<'src>),
    Return(ReturnStmt<'src>),
}

#[derive(Debug, Clone)]
pub struct LetStmt<'src> {
    pub name: Token<'src>,
    pub expr: Option<Expression<'src>>,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt<'src> {
    pub expr: Option<Expression<'src>>,
}

impl Statement<'_> {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(_) => "let",
            Statement::Return(_) => "return",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Program<'src> {
    pub statements: Vec<Statement<'src>>,
}

impl Program<'_> {
    pub fn token_literal(&self) -> &str {
        self.statements
            .first()
            .map(|s| s.token_literal())
            .unwrap_or("")
    }
}
