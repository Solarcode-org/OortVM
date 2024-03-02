use logos::Logos;

use crate::error::error;
use crate::error::get_token;

use super::access;
use super::compile;
use super::lexer;

#[derive(PartialEq, Debug)]
pub(crate) enum Expr {
    _Integer(i64),
    _Add(Box<Expr>, Box<Expr>),
    _Subtract(Box<Expr>, Box<Expr>),
    _Multiply(Box<Expr>, Box<Expr>),
    _Divide(Box<Expr>, Box<Expr>),
    Func(compile::Compile, Box<Expr>),
    Args(Vec<Expr>),
}

pub(crate) fn parse(ir_line: String) -> Expr {
    let mut ast = vec![];
    let mut describe = String::new();

    let tokens = lexer::Token::lexer(&ir_line);

    for token in tokens {
        let token = get_token(token);

        match token {
            lexer::Token::Descriptor(des) => describe = des,

            lexer::Token::Ident(ident) => {
                if describe.is_empty() {
                    error("Expected descriptor befor identifier");
                }

                match describe.as_str() {
                    "func" => {
                        if access::funcs_contains(&ident) {
                            let f = access::funcs_get(ident).unwrap();

                            ast.push(Expr::Func(f, Box::new(Expr::Args(vec![]))))
                        }
                    }
                    des => error(format!("Unknown descriptor: {des}").as_str()),
                }
            }

            _ => error("Expected descriptor token `%` or Identifier"),
        }
    }

    Expr::Args(ast)
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parser() {
        let ast = parse("%func print".to_string());

        let f = access::funcs_get("print".to_string()).unwrap();
        let expectedf = Expr::Func(f, Box::new(Expr::Args(vec![])));

        assert_eq!(ast, Expr::Args(vec![expectedf]));
    }
}