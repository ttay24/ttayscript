use std::iter::Peekable;
use std::vec::IntoIter;
use std::{fs, io};
use log::trace;

use crate::domain::dto::{Result};
use crate::lexer::{VALID_OPERATORS, VALID_SEPARATORS, VALID_SYMBOLS};
use super::token::Token;
use super::{Literal, KEYWORDS};

pub struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
}

impl Lexer {
    pub fn from_text(text: &str) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
        }
    }

    pub fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(Self::from_text(&fs::read_to_string(file_path)?))
    }

    /// Create a token by eating characters while a condition is met.
    ///
    /// # Arguments
    /// * `raw_token` - The raw string token to append characters to.
    /// * `cond` - The condition that must be met.
    fn get_next_char_while(&mut self, raw_token: &mut String, cond: fn(char) -> bool) {
        loop {
            match self.raw_data.peek() {
                Some(c) if cond(*c) => {
                    raw_token.push(*c);
                    self.raw_data.next();
                }
                _ => {
                    trace!(
                        "Stopping get_next_char_while after peeking {:?}",
                        self.raw_data.peek()
                    );
                    break;
                }
            }
        }
    }

    fn is_identifier(c: char) -> bool {
        c.is_ascii_alphanumeric() || c =='_'
    }

    fn is_keyword(identifier: &String) -> bool {
        KEYWORDS.contains(&&identifier[..])
    }
}

impl Iterator for Lexer {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let token: Result<Token>;

        let first_char: char;

        loop {
            match self.raw_data.next() {
                Some(c) if c.is_whitespace() => continue,
                Some(c) => {
                    first_char = c;
                    break;
                }
                None => return None,
            }
        }

        trace!("First char: {}", first_char);

        // Identifier or keyword
        if Self::is_identifier(first_char) && !first_char.is_numeric() {
            trace!("Lexing identifier");
            let mut name = first_char.to_string();
            self.get_next_char_while(&mut name, Self::is_identifier);

            if (Self::is_keyword(&name)) {
                token = Ok(Token::Keyword(name))
            }
            else {
                token = Ok(Token::Identifier(name));
            }
        }
        // Integer Literal
        else if first_char.is_numeric() {
            trace!("Lexing integer literal");
            let mut value = first_char.to_string();
            self.get_next_char_while(&mut value, |c| c.is_numeric());

            token = match value.parse() {
                Ok(i) => Ok(Token::Literal(Literal::Integer(i))),
                Err(_) => Err(format!("Integer literal {} is invalid", value)),
            }
        }
        // String Literal
        else if first_char == '"' {
            trace!("Lexing string literal");
            let mut value = String::new();

            self.get_next_char_while(&mut value, |c| c != '"');
            self.raw_data.next(); // Eat ending "

            token = Ok(Token::Literal(Literal::Str(value)));
        }
        // Boolean literal
        // TODO
        // Symbol
        else {
            trace!("Lexing symbol");
            let mut raw = first_char.to_string();
            loop {
                if let Some(peek) = self.raw_data.peek() {
                    raw.push(*peek);
                } else {
                    break;
                }

                if VALID_OPERATORS.contains(&&raw[..]) {
                    self.raw_data.next();
                }
                else if VALID_SEPARATORS.contains(&&raw[..]) {
                    self.raw_data.next();
                }
                else if VALID_SYMBOLS.contains(&&raw[..]) {
                    self.raw_data.next();
                } else {
                    raw.pop();
                    break;
                }
            }

            token = match &raw[..] {
                // Ignore comments until newline
                s if s == "//" => {
                    trace!("Ignoring comment");
                    self.get_next_char_while(&mut String::new(), |c| c != '\n');
                    self.next()?
                }
                s if VALID_OPERATORS.contains(&s) => Ok(Token::Operator(raw)),
                s if VALID_SEPARATORS.contains(&s) => Ok(Token::Separator(raw)),
                s if VALID_SYMBOLS.contains(&s) => Ok(Token::Symbol(raw)),
                _ => Err(format!("Unknown token: {}", raw)),
            }
        }

        trace!("");

        Some(token)
    }
}