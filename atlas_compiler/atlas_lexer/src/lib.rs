use atlas_core::prelude::*;
lexer_builder!();
symbols!(
    '=' => Equal,
    '+' => Plus,
    '-' => Minus,
    '/' => Slash,
    ';' => Semicolon,
    '*' => Star,
    '%' => Percentage,
    '?' => Question,
    ',' => Comma,
    ':' => Colon,
    '.' => Dot,
    '{' => LBrace,
    '}' => RBrace,
    '[' => LBracket,
    ']' => RBracket,
    '(' => LParenthesis,
    ')' => RParenthesis,
    '&' => Ampersand,
    '>' => RAngle,
    '<' => LAngle,
    '!' => Not
);
keywords!(
    //Reserved keywords:
    "if",
    "else",
    "while",
    "loop",
    "function",
    "let",
    "struct",
    "enum",
    "union",
    "class",
    "public",
    "private",
    "trait",
    "implements",
    "extern",
    "type",
    "import",
    //Types:
    //NB: The types will, later, be removed and treated as identifier first (because `i32()` could exist)
    "Self",
    "i8",
    "i16",
    "i32",
    "i64",
    "u8",
    "u16",
    "u32",
    "u64",
    "f32",
    "f64",
    "char",
    "string",
    "vector",
);

pub fn comment_system(c: char, state: &mut LexerState) -> Option<Token> {
    if c == ';' {
        let start = state.current_pos;
        state.next();
        while let Some(c) = state.peek() {
            if *c == '\n' {
                break;
            }
            state.next();
        }
        return Some(Token {
            span: Span {
                start,
                end: state.current_pos,
                path: state.path,
            },
            kind: TokenKind::WhiteSpace,
        });
    }

    None
}

pub fn string_literal_system(c: char, state: &mut LexerState) -> Option<Token> {
    if c == '"' {
        let start = state.current_pos;
        let mut s = String::new();
        s.push(c);
        state.next();
        while let Some(c) = state.peek() {
            if *c != '"' {
                s.push(*c);
                state.next();
            } else {
                break;
            }
        }
        state.next();
        return Some(Token::new(
            Span {
                start,
                end: state.current_pos,
                path: state.path,
            },
            TokenKind::Literal(Literal::StringLiteral(Intern::new(s))),
        ));
    }
    None
}

pub fn identifier_system(c: char, state: &mut LexerState) -> Option<Token> {
    if c.is_alphabetic() || c == '_' {
        let start = state.current_pos;
        let mut s = String::new();
        s.push(c);
        state.next();
        while let Some(c) = state.peek() {
            if c.is_alphabetic() || *c == '_' {
                s.push(*c);
                state.next();
            } else {
                break;
            }
        }
        return Some(Token::new(
            Span {
                start,
                end: state.current_pos,
                path: state.path,
            },
            TokenKind::Literal(Literal::Identifier(Intern::new(s))),
        ));
    }
    None
}
