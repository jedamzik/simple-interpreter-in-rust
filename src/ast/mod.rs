use std::fmt;
use crate::types::*;

// TODO:
// give Nodes a children attribute, extend BinaryOperator from that
// see Rust implementation by Ruslan Spivak: https://github.com/rspivak/lsbasi/blob/master/part7/rust/spi/src/main.rs

#[derive(Debug, Eq, PartialEq)]
pub struct AST {
    pub root: Node
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => f.write_str(&format!("{:?}", self))
        }
    }
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