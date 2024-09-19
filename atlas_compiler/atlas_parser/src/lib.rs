use ast::Declaration;
use atlas_core::utils::span::Spanned;
use atlas_lexer::{Literal, Token, TokenKind};
use internment::Intern;

pub mod ast;
pub mod class;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::class::*;
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    file: &'static str,
}

pub fn parse(file: &'static str, tokens: Vec<Token>) -> Result<Vec<Declaration>, ()> {
    let mut vec = vec![];
    let mut parser = Parser {
        file,
        tokens,
        index: 0,
    };
    loop {
        if parser.current_kind() == TokenKind::EoI {
            break;
        }
        let start_pos = parser.current_token().start();
    }

    Ok(vec)
}

impl Parser {
    #[must_use]
    #[inline(always)]
    pub fn current_kind(&self) -> TokenKind {
        self.tokens[self.index].kind()
    }
    #[must_use]
    #[inline(always)]
    pub fn current_token(&self) -> &Token {
        &self.tokens[self.index]
    }
    #[must_use]
    #[inline(always)]
    pub fn advance(&mut self) -> &Token {
        self.index += 1;
        &self.tokens[self.index]
    }
    #[must_use]
    #[inline(always)]
    pub fn retreat(&mut self) -> &Token {
        self.index -= 1;
        &self.tokens[self.index]
    }
    #[must_use]
    #[inline(always)]
    pub fn peek_token(&self) -> &Token {
        &self.tokens[self.index + 1]
    }
    #[must_use]
    #[inline(always)]
    pub fn peek_kind(&self) -> TokenKind {
        self.tokens[self.index + 1].kind()
    }
    #[must_use]
    #[inline(always)]
    pub fn expect_ident(&self) -> Result<Intern<String>, ()> {
        if let TokenKind::Literal(Literal::Identifier(v)) = self.current_kind() {
            Ok(v)
        } else {
            Err(())
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn expect(&self, kind: TokenKind) -> Result<(), ()> {
        if self.current_kind() == kind {
            Ok(())
        } else {
            Err(())
        }
    }
}
