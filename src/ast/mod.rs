use std::fmt;
use crate::types::*;

#[derive(Debug, Eq, PartialEq)]
pub struct AST {
    pub root: Node
}

#[derive(Debug, Eq, PartialEq)]
pub struct BinaryOperator {
    pub left: Node,
    pub token: Token,
    pub right: Node
}

#[derive(Debug, Eq, PartialEq)]
pub enum Node {
    Token(Token),
    BinaryOperator(Box<BinaryOperator>)
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => f.write_str(&format!("{:?}", self))
        }
    }
}