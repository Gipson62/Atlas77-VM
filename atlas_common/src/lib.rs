use internment::Intern;
use std::sync::Arc;

pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
    Bool,
    Struct(Intern<String>, Arc<[DataType]>),
    Class(Intern<String>, Arc<[DataType]>),
    Enum(Intern<String>, Arc<[Intern<String>]>),
}
