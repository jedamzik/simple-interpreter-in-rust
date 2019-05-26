use crate::types::*;
use crate::ast::*;
use crate::utils::*;

#[derive(Debug)]
struct Parser {
    current_token_index: usize,
    tokens: Vec<Token>
}

impl Parser {
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

    fn factor(&mut self) -> Node {
        self.skip_whitespace();

        let token = self.tokens[self.current_token_index];

        match token {
            Token::Number(Number::Integer(value)) => {
                self.eat(Token::Number(Number::Integer(value)));

                Node::Token(token)
            },
            Token::LPAREN => {
                self.eat(Token::LPAREN);

                let node = self.expr(); 

                self.eat(Token::RPAREN);

                node
            },
            _ => panic!(format!("Syntax Error: expected Integer or \"(\", found {}", token))
        }
    }

    fn expr(&mut self) -> Node {
        let mut node = self.term();

        while is_addsub_operator(self.tokens[self.current_token_index]) ||
              is_whitespace(self.tokens[self.current_token_index]) {
            let token = self.tokens[self.current_token_index];

            match token {
                Token::Operator(Operator::Add) => {
                    self.eat(token);
                    node = Node::BinaryOperator(Box::new(BinaryOperator {
                        left: node,
                        token,
                        right: self.term()
                    }))
                },
                Token::Operator(Operator::Sub) => {
                    self.eat(token);
                    node = Node::BinaryOperator(Box::new(BinaryOperator {
                        left: node,
                        token,
                        right: self.term()
                    }))
                },
                Token::Whitespace => self.skip_whitespace(),
                _ => panic!("Syntax Error: expected \"+\" or \"-\".")
            }
        }

        node
    }

    fn term(&mut self) -> Node {
        let mut node = self.factor();

        while is_muldiv_operator(self.tokens[self.current_token_index]) ||
              is_whitespace(self.tokens[self.current_token_index]) {
                let token = self.tokens[self.current_token_index];

                match token {
                    Token::Operator(Operator::Mul) => {
                        self.eat(token);
                        node = Node::BinaryOperator(Box::new(BinaryOperator {
                            left: node,
                            token,
                            right: self.term()
                        }))
                    },
                    Token::Operator(Operator::Div) => {
                        self.eat(token);
                        node = Node::BinaryOperator(Box::new(BinaryOperator {
                            left: node,
                            token,
                            right: self.term()
                        }))
                    },
                    Token::Whitespace => self.skip_whitespace(),
                    _ => panic!("Syntax Error: expected \"*\" or \"/\".")
                }
              }
        
        node
    }
}

pub fn parse(tokens: Vec<Token>) -> AST {
    let parser = &mut Parser {
        current_token_index: 0,
        tokens
    };

    AST {
        root: parser.expr()
    }
}

#[cfg(test)]
mod parser {
    // TODO:
    // - don't use lex but pass vector of tokens directly
    // - put these in their own test_file
    use crate::lexer::lex;
    use crate::parser::*;

    #[test]
    fn next_token_increments_current_token_index() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let parser = &mut Parser {
            current_token_index: 0,
            tokens
        };
        assert_eq!(parser.current_token_index, 0);

        parser.next_token();
        assert_eq!(parser.current_token_index, 1);
    }

    #[test]
    fn next_token_increment_is_clamped_to_length_of_token_vector() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let parser = &mut Parser {
            current_token_index: 0,
            tokens
        };
        assert_eq!(parser.current_token_index, 0);

        for _ in 0..12 {
            parser.next_token();
        }

        assert_eq!(parser.current_token_index, 8);
    }

    #[test]
    #[should_panic]
    fn eat_returns_error_if_the_passed_token_is_not_the_current_token() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let parser = &mut Parser {
            current_token_index: 0,
            tokens
        };

        parser.eat(Token::Whitespace);
    }

    #[test]
    fn eat_returns_current_token_if_passed_token_matches() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let parser = &mut Parser {
            current_token_index: 0,
            tokens
        };

        assert_eq!(parser.eat(Token::Number(Number::Integer(3))), Token::Number(Number::Integer(3)));
    }

    #[test]
    fn eat_advances_current_token_index() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let parser = &mut Parser {
            current_token_index: 0,
            tokens
        };
        assert_eq!(parser.current_token_index, 0);

        parser.eat(Token::Number(Number::Integer(3)));
        assert_eq!(parser.current_token_index, 1);
    }

    #[test]
    fn factor_skips_whitespace_tokens() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let parser = &mut Parser {
            current_token_index: 3,
            tokens
        };
        assert_eq!(parser.current_token_index, 3);

        parser.factor();

        assert_eq!(parser.current_token_index, 5);
    }

    #[test]
    fn factor_returns_number_token_as_node() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let parser = &mut Parser {
            current_token_index: 0,
            tokens
        };
        let node = parser.factor();
        assert_eq!(node, Node::Token(Token::Number(Number::Integer(3))));
    }

    #[test]
    #[should_panic]
    fn factor_throws_syntax_error_on_operator() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let parser = &mut Parser {
            current_token_index: 2,
            tokens
        };
        parser.factor();
    }

    #[test]
    fn factor_consumes_the_token() {
        let expr = "3 * 2 / 1";
        let tokens = lex(expr);
        let parser = &mut Parser {
            current_token_index: 0,
            tokens
        };
        assert_eq!(parser.current_token_index, 0);

        parser.factor();
        assert_eq!(parser.current_token_index, 1);
    }
}
