use crate::{ast, Lexer, Token};
use ast::Expr;

pub struct Parser {
    ts: Lexer,
}

impl Parser {
    pub fn new(ts: Lexer) -> Self {
        Self { ts }
    }

    pub fn parse(mut self) -> ParseResult<Vec<Expr>> {
        let mut exprs = Vec::new();

        while !self.ts.empty() {
            exprs.push(self.parse_top_level_expr()?);
        }

        Ok(exprs)
    }

    /// At the top-level (not inside a closure, decl, compound)
    /// we accept very few things:
    /// - Let declarations
    /// - struct definitions (TODO)
    /// - Enum declarations (TODO)
    fn parse_top_level_expr(&mut self) -> ParseExprResult {
        let token = self.ts.peek_next().ok_or(ParseError::UnexpectedEOF)?;

        match token {
            Token::Let => self.parse_let_decl(),
            _ => Err(ParseError::InvalidTopLevel(token)),
        }
    }

    fn parse_let_decl(&self) -> ParseExprResult {
        self.expect(Token::Let)?;

        let ident_decl = self.parse_ident_decl();
        let rhs = self.parse_expr();

        todo!()
    }

    fn parse_ident_decl(&mut self) -> ParseExprResult {
        let ident_spelling = match self.expect_id()? {
            Token::Id(s) => s,
            _ => unreachable!("Already Expected"),
        };

        let ty = self.parse_type();
        // Ok(ast::IdentDecl { ident, ty })
    }

    fn expect(&mut self, token: Token) -> ParseResult<Token> {
        match self.ts.next() {
            Some(t) if t.same_variant(token) => Ok(t),
            Some(t) => Err(ParseError::Expected(token, t)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn expect_id(&mut self) -> ParseResult<Token> {
        self.expect(Token::Id(String::new()))
    }
}

type ParseResult<T> = Result<T, ParseError>;
type ParseExprResult = ParseResult<Expr>;

enum ParseError {
    UnexpectedEOF,
    InvalidTopLevel(Token),
    Expected(Token, Token),
}
