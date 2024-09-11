use crate::ast::Visibility;
use atlas_common::DataType;
use internment::Intern;

pub struct ClassDeclaration {
    pub name: Intern<String>,
    pub fields: Vec<ClassField>,
}

pub struct ClassField {
    pub visibility: Visibility,
    pub name: Intern<String>,
    pub type_: DataType,
}
