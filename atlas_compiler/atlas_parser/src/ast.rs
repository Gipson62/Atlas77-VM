use internment::Intern;

use crate::class::ClassDeclaration;

#[repr(u8)]
pub enum Visibility {
    Private,
    Public,
}

pub enum Num {
    Integer(i64),
    Floating(f64)
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