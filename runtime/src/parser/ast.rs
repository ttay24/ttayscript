use crate::{Token, Literal, lexer::Operator};

#[derive(Debug)]
pub enum Node {
    Literal(Literal),
    UnaryExpr {
        op: Operator
    }
}

/*impl Node {
    pub fn new(token: Token, left: Option<Node>, right: Option<Node>) -> Self {
        Self {
            Literal
        }
    }
}*/

#[derive(Debug)]
pub struct AST {
    root: Node,
}

impl AST {
    pub fn new(token: Token) -> Self {
        Self {
            //root: Node::new(token),
            root: Node::Literal(Literal::Integer(24)),
        }
    }
}