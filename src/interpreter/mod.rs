use crate::types::*;
use crate::utils::clamp;

struct Interpreter {
    current_token_index: usize,
    tokens: Vec<Token>
}

impl Interpreter {
    fn next_token(&mut self) {
        self.current_token_index = clamp(self.current_token_index + 1, 0, self.tokens.len() - 1);
    }

    fn eat(&mut self, token: Token) -> Token {
        let current_token = self.tokens[self.current_token_index];

        if token != current_token {
            panic!("Syntax Error: expected {}, found {}", token, current_token)
        }

        self.next_token();

        current_token
    }

    fn skip_whitespace(&mut self) {
        if let Token::Whitespace = self.tokens[self.current_token_index] {
            self.current_token_index += 1;
        }
    }

    fn factor(&mut self) -> i32 {
        self.skip_whitespace();

        let token = self.tokens[self.current_token_index];

        match token {
            Token::Number(Number::Integer(value)) => {
                self.eat(Token::Number(Number::Integer(value)));

                value
            },
            Token::LPAREN => {
                self.eat(Token::LPAREN);

                let result = self.expr(); 

                self.eat(Token::RPAREN);

                result
            },
            _ => panic!(format!("Syntax Error: expected Integer or \"(\", found {}", token))
        }
    }

    fn expr(&mut self) -> i32 {
        let mut result = self.term();

        while is_addsub_operator(self.tokens[self.current_token_index]) ||
              is_whitespace(self.tokens[self.current_token_index]) {
            match self.tokens[self.current_token_index] {
                Token::Operator(Operator::Add) => {
                    self.eat(Token::Operator(Operator::Add));
                    result = result + self.term()
                },
                Token::Operator(Operator::Sub) => {
                    self.eat(Token::Operator(Operator::Sub));
                    result = result - self.term()
                },
                Token::Whitespace => self.skip_whitespace(),
                _ => panic!("Syntax Error: expected \"+\" or \"-\".")
            }
        }

        result
    }

    fn term(&mut self) -> i32 {
        let mut result: i32 = self.factor();

        while is_muldiv_operator(self.tokens[self.current_token_index]) ||
              is_whitespace(self.tokens[self.current_token_index]) {
                  match self.tokens[self.current_token_index] {
                      Token::Operator(Operator::Mul) => {
                          self.eat(Token::Operator(Operator::Mul));
                          result = result * self.factor();
                      },
                      Token::Operator(Operator::Div) => {
                          self.eat(Token::Operator(Operator::Div));
                          result = result / self.factor();
                      },
                      Token::Whitespace => self.skip_whitespace(),
                      _ => panic!("Syntax Error: expected \"*\" or \"/\".")
                  }
              }
        
        result
    }
}

pub fn interpret(tokens: Vec<Token>) -> String {
    let interpreter = &mut Interpreter {
        current_token_index: 0,
        tokens
    };

    let result_string = String::from(format!("{}", interpreter.expr()));
    result_string
}

#[cfg(test)]
mod interpreter {
    // TODO:
    // - don't use lex but pass vector of tokens directly
    // - put these in their own test_file
    use crate::lexer::lex;
    use crate::interpreter::*;

    #[test]
    fn next_token_increments_current_token_index() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        assert_eq!(interpreter.current_token_index, 0);

        interpreter.next_token();
        assert_eq!(interpreter.current_token_index, 1);
    }

    #[test]
    fn next_token_increment_is_clamped_to_length_of_token_vector() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        assert_eq!(interpreter.current_token_index, 0);

        for _ in 0..12 {
            interpreter.next_token();
        }

        assert_eq!(interpreter.current_token_index, 8);
    }

    #[test]
    #[should_panic]
    fn eat_returns_error_if_the_passed_token_is_not_the_current_token() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };

        interpreter.eat(Token::Whitespace);
    }

    #[test]
    fn eat_returns_current_token_if_passed_token_matches() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };

        assert_eq!(interpreter.eat(Token::Number(Number::Integer(3))), Token::Number(Number::Integer(3)));
    }

    #[test]
    fn eat_advances_current_token_index() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        assert_eq!(interpreter.current_token_index, 0);

        interpreter.eat(Token::Number(Number::Integer(3)));
        assert_eq!(interpreter.current_token_index, 1);
    }

    #[test]
    fn factor_skips_whitespace_tokens() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 3,
            tokens
        };
        assert_eq!(interpreter.current_token_index, 3);

        interpreter.factor();

        assert_eq!(interpreter.current_token_index, 5);
    }

    #[test]
    fn factor_returns_integer_value() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        let result = interpreter.factor();
        assert_eq!(result, 3);
    }

    #[test]
    #[should_panic]
    fn factor_throws_syntax_error_on_operator() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 2,
            tokens
        };
        interpreter.factor();
    }

    #[test]
    fn factor_eats_the_token() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        assert_eq!(interpreter.current_token_index, 0);

        interpreter.factor();
        assert_eq!(interpreter.current_token_index, 1);
    }

    #[test]
    fn multiplication_gets_precedence_before_addition() {
        let expr = "3 + 2 * 5";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        let result = interpreter.expr();
        assert_eq!(result, 13);
    }

    #[test]
    fn multiplication_gets_precedence_before_subtraction() {
        let expr = "3 - 2 * 5";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        let result = interpreter.expr();
        assert_eq!(result, -7);
    }

    #[test]
    fn division_gets_precedence_before_addition() {
        let expr = "3 + 10 / 5";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        let result = interpreter.expr();
        assert_eq!(result, 5);
    }

    #[test]
    fn division_gets_precedence_before_subtraction() {
        let expr = "3 - 6 / 2";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        let result = interpreter.expr();
        assert_eq!(result, 0);
    }

    #[test]
    fn parenthesized_expressions_get_precedence_on_left_hand_of_operator() {
        let expr = "(200 + 50) * 3";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        let result = interpreter.expr();
        assert_eq!(result, 750);
    }

    #[test]
    fn parenthesized_expressions_get_precedence_on_right_hand_of_operator() {
        let expr = "3 * (100 + 50)";
        let tokens = lex(expr);
        let interpreter = &mut Interpreter {
            current_token_index: 0,
            tokens
        };
        let result = interpreter.expr();
        assert_eq!(result, 450);
    }
}