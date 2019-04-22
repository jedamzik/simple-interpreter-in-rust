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

mod lexer;
mod interpreter;
mod types;
mod utils;

fn main() {
    let buffer = &mut String::new();
    match io::stdin().read_line(buffer) {
        Ok(_) => {
            let expr = buffer.trim_end();
            let tokens = lexer::lex(expr);
            let result = interpreter::interpret(tokens);
            println!("= {}", result);
        },
        Err(e) => panic!(e)
    }
}
