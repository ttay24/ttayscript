use super::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Literal(Literal),
    Symbol(String),
    Keyword(String),

    // symbol broken out
    Operator(String),
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