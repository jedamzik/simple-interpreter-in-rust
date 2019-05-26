use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    Number(Number),
    Operator(Operator),
    Whitespace,
    LPAREN,
    RPAREN,
    Unknown
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Number {
    Integer(i32)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => f.write_str(&format!("{:?}", self))
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => f.write_str(&format!("{:?}", self))
        }
    }
}

pub fn is_addsub_operator(token: Token) -> bool {
    match token {
        Token::Operator(Operator::Add) => true,
        Token::Operator(Operator::Sub) => true,
        _ => false
    }
}

pub fn is_muldiv_operator(token: Token) -> bool {
    match token {
        Token::Operator(Operator::Mul) => true,
        Token::Operator(Operator::Div) => true,
        _ => false
    }
}

pub fn is_whitespace(token: Token) -> bool {
    if let Token::Whitespace = token {
        return true
    }
    false
}