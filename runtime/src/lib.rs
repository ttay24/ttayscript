mod core;

mod domain;
pub use domain::dto;
pub use domain::enums;

mod interpreter;
pub use interpreter::Interpreter;

mod lexer;
pub use lexer::Lexer;
pub use lexer::Literal;
pub use lexer::Token;

mod parser;
pub use parser::Parser;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
