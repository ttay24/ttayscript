use std::io;

use crate::services::Lexer;

pub struct Interpreter {
    lexer: Lexer,
}

impl Interpreter {
    pub fn new(file: &str) -> Self {
        // assume it's a file
        Self::from_file(file).unwrap()
    }

    fn from_text(text: &str) -> Self {
        Interpreter {
            lexer: Lexer::from_text(&text),
        }
    }

    fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(
            Self {
                lexer: Lexer::from_file(&file_path).unwrap()
            }
        )
    }

    pub fn run(self) -> Option<String> {
        let mut lexer_iter = self.lexer.into_iter();
        let result = lexer_iter.next();
        
        // test outputting the strings
        /*for token in self.lexer {
            match token {
                Ok(t) => println!("{:?}", t),
                Err(e) => println!("Error! {}", e),
            }
        }*/

        while let Some(ref mut test) = lexer_iter.next() {
            match test {
                Ok(t) => println!("{:?}", t),
                Err(e) => println!("Error! {}", e),
            }
        }

        // do expression stuff

        None
    }

    fn expr(self) {}
}