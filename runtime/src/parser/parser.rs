use std::io;

use crate::Lexer;
use crate::domain::dto::{Result};

use super::{AST,Node};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    ast: String
}

impl Parser {
    pub fn from_text(text: &str) -> Self {
        Self {
            lexer: Lexer::from_text(&text),
            ast: String::from("test")
        }
    }

    pub fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(
            Self {
                lexer: Lexer::from_file(&file_path).unwrap(),
                ast: String::from("test")
            }
        )
    }

    pub fn parse(self) -> Result<AST> {

        // setup the lexer
        let mut lexer_iter = self.lexer.into_iter();
        let result = lexer_iter.next();
        
        // test outputting the strings
        /*for token in self.lexer {
            match token {
                Ok(t) => println!("{:?}", t),
                Err(e) => println!("Error! {}", e),
            }
        }*/

        // check for errors
        if (result.is_none()) {
            return Err(String::from("Something went wrong parsing"));
        }

        // setup AST response
        let ast: AST;

        match result.unwrap() {
            Ok(token) => {
                println!("{:?}", token);
                ast = AST::new(token);
            },
            Err(e) => {
                println!("Error! {}", e);
                return Err(e);
            },
        }

        while let Some(ref mut test) = lexer_iter.next() {
            match test {
                Ok(t) => {
                    println!("{:?}", t);
                    // TODO: more parsing things
                },
                Err(e) => println!("Error! {}", e),
            }
        }

        // do expression stuff

        Ok(ast)
    }

    fn expr(self) {}
}