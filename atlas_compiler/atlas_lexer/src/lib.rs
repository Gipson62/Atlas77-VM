use atlas_core::prelude::*;
lexer_builder!();
symbols!(
    '=' => Equal
);
keywords!(
    //reserved keyword
    "if",
    "else",
    "while",
    "loop",
    "function",
    "let",
    "struct",
    "enum",
    "union",
    "public",
    "trait",
    "implements",
    "extern",
    "type",
    //types
    "int",
    "u_int",
    "string",
    "char",
);
