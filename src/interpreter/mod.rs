use crate::types::*;
use crate::ast::*;

struct Interpreter {}

impl Interpreter {
    fn visit(&self, node: &Node) -> i32 {
        match node {
            Node::BinaryOperator(node) => {
                let result: i32;
                match node.token {
                    Token::Operator(Operator::Add) => result = self.visit(&node.left) + self.visit(&node.right),
                    Token::Operator(Operator::Sub) => result = self.visit(&node.left) - self.visit(&node.right),
                    Token::Operator(Operator::Mul) => result = self.visit(&node.left) * self.visit(&node.right),
                    Token::Operator(Operator::Div) => result = self.visit(&node.left) / self.visit(&node.right),
                    _ => panic!(format!("AST Traversal Error: expected Operator, found {}.", node.token))
                }
                result
            },
            Node::Token(node) => {
                let result: i32;
                match node {
                    Token::Number(Number::Integer(value)) => result = *value,
                    _ => panic!(format!("AST Traversal Error: expected Number, found {}", node))
                }

                result
            }
        }
    }
}

pub fn interpret(ast: &AST) -> i32 {
    let interpreter = Interpreter {};

    interpreter.visit(&ast.root)
}

#[cfg(test)]
mod interpreter {
    use crate::lexer::*;
    use crate::parser::*;
    use crate::interpreter::*;

    #[test]
    fn multiplication_gets_precedence_before_addition() {
        let expr = "3 + 2 * 5";
        let tokens = lex(expr);
        let ast = parse(tokens);
        let result = interpret(&ast);

        assert_eq!(result, 13);
    }

    #[test]
    fn multiplication_gets_precedence_before_subtraction() {
        let expr = "3 - 2 * 5";
        let tokens = lex(expr);
        let ast = parse(tokens);
        let result = interpret(&ast);
        assert_eq!(result, -7);
    }

    #[test]
    fn division_gets_precedence_before_addition() {
        let expr = "3 + 10 / 5";
        let tokens = lex(expr);
        let ast = parse(tokens);
        let result = interpret(&ast);
        assert_eq!(result, 5);
    }

    #[test]
    fn division_gets_precedence_before_subtraction() {
        let expr = "3 - 6 / 2";
        let tokens = lex(expr);
        let ast = parse(tokens);
        let result = interpret(&ast);
        assert_eq!(result, 0);
    }

    #[test]
    fn parenthesized_expressions_get_precedence_on_left_hand_of_operator() {
        let expr = "(200 + 50) * 3";
        let tokens = lex(expr);
        let ast = parse(tokens);
        let result = interpret(&ast);
        assert_eq!(result, 750);
    }

    #[test]
    fn parenthesized_expressions_get_precedence_on_right_hand_of_operator() {
        let expr = "3 * (100 + 50)";
        let tokens = lex(expr);
        let ast = parse(tokens);
        let result = interpret(&ast);
        assert_eq!(result, 450);
    }
}