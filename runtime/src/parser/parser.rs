use std::io;

use log::{error, info};

use crate::Lexer;
use crate::domain::dto::{Result};

use super::{AST,Node};

#[derive(Debug)]
enum ParseType {
    File,
    Text,
}

#[derive(Debug)]
pub struct Parser {
    //lexer: Lexer,
    text: String,
    parse_type: ParseType,
    ast: String
}

impl Parser {
    pub fn from_text(text: &str) -> Self {
        Self {
            text: String::from(text),
            parse_type: ParseType::Text,
            ast: String::from("test")
        }
    }

    pub fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(
            Self {
                text: String::from(file_path),
                parse_type: ParseType::File,
                ast: String::from("test")
            }
        )
    }

    pub fn parse(self) -> Result<AST> {
        let lexer = match self.parse_type {
            ParseType::File => Lexer::from_file(&self.text).unwrap(),
            ParseType::Text => Lexer::from_text(&self.text),
        };

        // setup the lexer
        let mut lexer_iter = lexer.into_iter();
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
                info!("{:?}", token);
                ast = AST::new(token);
            },
            Err(e) => {
                error!("Error! {}", e);
                return Err(e);
            },
        }

        while let Some(ref mut test) = lexer_iter.next() {
            match test {
                Ok(t) => {
                    info!("{:?}", t);
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