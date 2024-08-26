#![allow(dead_code)]
pub mod instruction;
pub mod memory;
pub mod runtime;

pub mod prelude {
    pub use crate::{
        instruction::{
            compiler::{lexer::*, parser::*},
            Address, Instruction,
        },
        memory::{object_map::*, stack::*, vm_data::VMData},
        runtime::{vm_state::VMState, CallBack, VM},
    };
    pub use internment::Intern;
}
