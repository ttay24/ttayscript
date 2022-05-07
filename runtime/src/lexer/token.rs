use super::literal::Literal;


#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Literal(Literal),
    Symbol(String),
    Keyword(String),

    // symbol broken out
    Operator(Operator),
    Separator(String),
}

/// Gets the precedence of an binary operation.
///
/// Higher number meaning higher precedence. If the operation is invalid, -1 is returned.
///
/// # Arguments
/// * `op` - The binary operation.
pub fn binary_op_precedence(op: &str) -> i32 {
    match op {
        "=" => 0,
        "==" | "!=" | "<" | ">" | "<=" | ">=" => 10,
        "+" | "-" => 20,
        "*" | "/" => 30,
        _ => -1,
    }
}

pub const KEYWORDS: &[&str] = &[
    "break", "continue", "fn", "for", "if", "return", "var", "while"
];

/*pub const VALID_OPERATORS: &[&str] = &[
    "=", "+", "-", "*", "/", "==", "!=", "<", ">", "<=", ">=",
    "{", "}", "[", "]", "(", ")", "//"
];*/
pub const VALID_OPERATORS: &[&str] = &[
    "=", "+", "-", "*", "/"
];

pub const VALID_SEPARATORS: &[&str] = &[
    "@", "->", ";", ",", "{", "}", "[", "]", "(", ")",
];

pub const VALID_SYMBOLS: &[&str] = &[
    "=", "+", "-", "*", "/", "==", "!=", "<", ">", "<=", ">=", "@", "->", ";", ",",
    "{", "}", "[", "]", "(", ")", "//"
];