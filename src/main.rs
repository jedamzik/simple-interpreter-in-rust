/**
 *
 * TODO:
 *
 * - handle div/mul cases with % (handle floating point calculation)
 * - syntax error for LPAREN w/o expr or RPAREN
 * - syntax error for RPAREN w/o opening LPAREN
 *
*/

use std::io;

mod ast;
mod types;
mod lexer;
mod parser;
mod interpreter;
mod utils;

fn main() {
    let buffer = &mut String::new();

    if io::stdin().read_line(buffer).is_ok() {
        let expr = buffer.trim_end();
        let tokens = lexer::lex(expr);
        let ast = parser::parse(tokens);
        let result = interpreter::interpret(&ast);
        println!("= {}", result);
    }
}
