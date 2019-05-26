extern crate strcursor;

use strcursor::StrCursor;
use crate::types::*;

fn tokenize_word(word: &str) -> Token {
    let integer: Result<i32, std::num::ParseIntError> = word.parse::<i32>();

    match integer {
        Ok(value) => Token::Number(Number::Integer(value)),
        Err(_e) => {
            match word {
                "+" => Token::Operator(Operator::Add),
                "-" => Token::Operator(Operator::Sub),
                "*" => Token::Operator(Operator::Mul),
                "/" => Token::Operator(Operator::Div),
                "(" => Token::LPAREN,
                ")" => Token::RPAREN,
                " " => Token::Whitespace,
                _ => Token::Unknown
            }
        }
    }
}

#[test]
fn tokenize_word_returns_integer_number() {
    let word = "123";
    let token = tokenize_word(&word);
    assert_eq!(token, Token::Number(Number::Integer(123)));
}

#[test]
fn tokenize_word_returns_add_operator() {
    let word = "+";
    let token = tokenize_word(&word);
    assert_eq!(token, Token::Operator(Operator::Add));
}

#[test]
fn tokenize_word_returns_sub_operator() {
    let word = "-";
    let token = tokenize_word(&word);
    assert_eq!(token, Token::Operator(Operator::Sub));
}

#[test]
fn tokenize_word_returns_mul_operator() {
    let word = "*";
    let token = tokenize_word(&word);
    assert_eq!(token, Token::Operator(Operator::Mul));
}

#[test]
fn tokenize_word_returns_div_operator() {
    let word = "/";
    let token = tokenize_word(&word);
    assert_eq!(token, Token::Operator(Operator::Div));
}

#[test]
fn tokenize_word_returns_whitespace() {
    let word = " ";
    let token = tokenize_word(&word);
    assert_eq!(token, Token::Whitespace);
}

#[test]
fn tokenize_word_returns_left_paren() {
    let word = "(";
    let token = tokenize_word(&word);
    assert_eq!(token, Token::LPAREN);
}

#[test]
fn tokenize_word_returns_right_paren() {
    let word = ")";
    let token = tokenize_word(&word);
    assert_eq!(token, Token::RPAREN);
}

fn is_integer(c: &str) -> bool {
    let integer: Result<i32, std::num::ParseIntError> = c.parse::<i32>();

    integer.is_ok()
}

#[test]
fn is_integer_returns_correctly() {
    let integer_string = "123";
    let float_string = "1.23";
    let string = "abc";

    assert_eq!(is_integer(integer_string), true);
    assert_eq!(is_integer(float_string), false);
    assert_eq!(is_integer(string), false);
}

fn parse_digits<'a>(cur: &mut strcursor::StrCursor, number_string: &'a mut String) -> &'a str {
    if let Some(gc) = cur.after() {
        let digit = &gc.to_string();
        if is_integer(digit) {
            number_string.push_str(digit);
            cur.seek_next();
            return parse_digits(cur, number_string)
        }
    } 

    number_string
}

#[test]
fn parse_digits_handles_one_digit_integers() {
    let text = String::from("1");
    let mut cur = StrCursor::new_at_start(&text); 
    let mut number_string = String::new();
    let integer_string = parse_digits(&mut cur, &mut number_string);

    assert_eq!(integer_string, "1");
}

#[test]
fn parse_digits_handles_two_digit_integers() {
    let text = String::from("12");
    let mut cur = StrCursor::new_at_start(&text); 
    let mut number_string = String::new();
    let integer_string = parse_digits(&mut cur, &mut number_string);

    assert_eq!(integer_string, "12");
}

#[test]
fn parse_digits_handles_three_digit_integers() {
    let text = String::from("123");
    let mut cur = StrCursor::new_at_start(&text); 
    let mut number_string = String::new();
    let integer_string = parse_digits(&mut cur, &mut number_string);

    assert_eq!(integer_string, "123");
}

fn tokenize(text: &str, tokens: &[Token]) -> Vec<Token> {
    let mut tokens = tokens.to_owned();

    let mut cur = StrCursor::new_at_start(text); 

    while let Some(gc) = cur.after() {
        if is_integer(&gc.to_string()) {
            tokens.push(tokenize_word(&parse_digits(&mut cur, &mut String::new())));
        } else {
            tokens.push(tokenize_word(&gc.to_string()));
            cur.seek_next()
        }
    }

    tokens
}

#[test]
fn tokenize_returns_a_vector_of_tokens() {
    let expr = "2 / 321 - 44 * 31";
    let tokens = tokenize(&expr, &Vec::new());

    assert_eq!(tokens, [
        Token::Number(Number::Integer(2)),
        Token::Whitespace,
        Token::Operator(Operator::Div),
        Token::Whitespace,
        Token::Number(Number::Integer(321)),
        Token::Whitespace,
        Token::Operator(Operator::Sub),
        Token::Whitespace,
        Token::Number(Number::Integer(44)),
        Token::Whitespace,
        Token::Operator(Operator::Mul),
        Token::Whitespace,
        Token::Number(Number::Integer(31))
    ]);
}

pub fn lex(text: &str) -> Vec<Token> {
    tokenize(text, &Vec::new())
}

#[test]
fn lex_returns_a_vector_of_tokens() {
    let expr = "2 + 3 * 4";
    let tokens = lex(expr);

    assert_eq!(tokens, [
        Token::Number(Number::Integer(2)),
        Token::Whitespace,
        Token::Operator(Operator::Add),
        Token::Whitespace,
        Token::Number(Number::Integer(3)),
        Token::Whitespace,
        Token::Operator(Operator::Mul),
        Token::Whitespace,
        Token::Number(Number::Integer(4))
    ]);
}