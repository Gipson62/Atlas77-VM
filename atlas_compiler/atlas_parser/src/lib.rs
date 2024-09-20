use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod class;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::class::*;
}

lalrpop_mod!(grammar);
