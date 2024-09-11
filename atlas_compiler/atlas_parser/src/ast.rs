use atlas_lexer::{Token, TokenKind};
use internment::Intern;

use crate::class::ClassDeclaration;

#[repr(u8)]
pub enum Visibility {
    Private,
    Public,
}

pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    StructDeclaration {
        name: Intern<String>,
    },
    ClassDeclaration(ClassDeclaration),
    ImportDeclaration {
        ///std::collection::string => ["std", "collection", "string"]
        path: Vec<Intern<String>>,
    },
    TypeDeclaration {
        name: Intern<String>,
        alias: Intern<String>,
    },
}
pub struct FunctionDeclaration {
    pub name: Intern<String>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    Equals,
    NotEquals,
    GreaterThan,
    LesserThan,
    GreaterEquals,
    LesserEquals,
}

impl From<Token> for BinaryOperator {
    fn from(value: Token) -> Self {
        match value.kind() {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Subtract,
            TokenKind::Star => Self::Multiply,
            TokenKind::Slash => Self::Divide,
            TokenKind::Percentage => Self::Modulo,
            TokenKind::RAngle => Self::LesserThan,
            TokenKind::LAngle => Self::GreaterThan,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
    Negate,
}

impl From<Token> for UnaryOperator {
    fn from(value: Token) -> Self {
        match value.kind() {
            TokenKind::Minus => Self::Negate,
            TokenKind::Not => Self::Not,
            _ => unimplemented!(),
        }
    }
}
