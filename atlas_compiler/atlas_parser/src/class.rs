use crate::ast::Visibility;
use crate::prelude::FunctionDeclaration;
use atlas_common::DataType;
use internment::Intern;

pub struct ClassDeclaration {
    pub visibility: Visibility,
    pub name: Intern<String>,
    pub fields: Vec<ClassField>,
    pub methods: Vec<FunctionDeclaration>,
}

pub struct ClassField {
    pub visibility: Visibility,
    pub name: Intern<String>,
    pub type_: DataType,
}
