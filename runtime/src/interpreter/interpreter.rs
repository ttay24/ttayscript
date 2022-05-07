use std::io;

use log::info;

use crate::{Parser, parser::AST};

pub struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    pub fn from_text(text: &str) -> Self {
        Self {
            parser: Parser::from_text(&text),
        }
    }

    pub fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(
            Self {
                parser: Parser::from_file(file_path).unwrap()
            }
        )
    }

    pub fn run(self) -> Option<String> {
        // parse the code and generate an AST
        let ast = self.parser.parse();

        match ast {
            Ok(ast) => {
                info!("handling ast! {:?}", &ast);
            },
            Err(e) => {
                info!("{}", e);
                return None;
            },
        }

        None
    }

    fn expr(self) {}
}