#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i32),
    Str(String),
}