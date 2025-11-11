use crate::{
    ast::{self, Statement},
    lexer::Lexer,
    token::Token,
};
use std::mem::discriminant;

pub struct Parser<'lex, 'src> {
    lexer: &'lex mut Lexer<'src>,
    curr_token: Token<'src>,
    peek_token: Token<'src>,
    errors: Vec<String>,
}

impl<'lex, 'src> Parser<'lex, 'src> {
    pub fn new(lexer: &'lex mut Lexer<'src>) -> Self {
        let mut parser = Parser {
            lexer,
            curr_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: Vec::<String>::new(),
        };

        // Load both curr and peek token
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token;
        self.peek_token = self.lexer.next_token()
    }

    pub fn parse_program(&mut self) -> ast::Program<'src> {
        let mut statements = Vec::<Statement>::new();
        while self.curr_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        ast::Program { statements }
    }

    pub fn parse_statement(&mut self) -> Option<ast::Statement<'src>> {
        match self.curr_token {
            Token::Let => match self.parse_let_statement() {
                Some(stmt) => Some(ast::Statement::Let(stmt)),
                None => None,
            },
            Token::Return => match self.parse_return_statement() {
                Some(stmt) => Some(ast::Statement::Return(stmt)),
                None => None,
            },
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<ast::LetStmt<'src>> {
        if !self.expect_peek_and_advance(Token::Identifier("")) {
            return None;
        }

        let stmt: ast::LetStmt<'_> = ast::LetStmt {
            name: self.curr_token,
            expr: None,
        };

        if !self.expect_peek_and_advance(Token::Assign) {
            return None;
        }

        while self.curr_token != Token::Semicolon {
            self.next_token();
        }

        Some(stmt)
    }

    pub fn parse_return_statement(&mut self) -> Option<ast::ReturnStmt<'src>> {
        let stmt = ast::ReturnStmt { expr: None };

        self.next_token();

        while self.curr_token != Token::Semicolon {
            self.next_token();
        }

        Some(stmt)
    }

    pub fn expect_peek_and_advance(&mut self, token: Token<'src>) -> bool {
        if discriminant(&self.peek_token) == discriminant(&token) {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }

    pub fn peek_error(&mut self, token: Token<'src>) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            token, self.peek_token
        );
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Statement;

    #[test]
    fn test_let_statements() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let tests = vec!["x", "y", "foobar"];

        for (i, expected_identifier) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            assert!(
                test_let_statement(stmt, expected_identifier),
                "test_let_statement failed for test {}",
                i
            );
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
return 5;
return 10;
return 993322;
"#;

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        for (i, stmt) in program.statements.iter().enumerate() {
            assert_eq!(
                stmt.token_literal(),
                "return",
                "test_return_statemnt failed for test {}",
                i
            );
        }
    }

    fn check_parser_errors(parser: &Parser) {
        if parser.errors.is_empty() {
            return;
        }

        eprintln!("parser has {} errors", parser.errors.len());
        for err in &parser.errors {
            eprintln!("{}", err); // item is &i32
        }
    }

    fn test_let_statement(stmt: &Statement, name: &str) -> bool {
        if stmt.token_literal() != "let" {
            eprintln!(
                "stmt.token_literal not 'let'. got='{}'",
                stmt.token_literal()
            );

            return false;
        }

        // Pattern match to check if it's a LetStatement
        let let_stmt = match stmt {
            Statement::Let(let_stmt) => let_stmt,
            _ => {
                eprintln!("stmt not LetStatement. got={:?}", stmt);

                return false;
            }
        };

        if let_stmt.name != Token::Identifier(name) {
            eprintln!("let_stmt.name not '{}'. got='{}'", name, let_stmt.name);

            return false;
        }

        true
    }
}
