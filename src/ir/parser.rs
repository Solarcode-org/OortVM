use std::collections::HashMap;
use std::io;
use std::io::{stdout, Write};

use logos::Logos;

use crate::error::error;
use crate::error::get_token;
use crate::ir::compile::{Compile, IRFunc};

use super::lexer;

#[derive(PartialEq, Debug)]
pub(crate) enum Expr {
    Number(f64),
    Func(Compile, Box<Expr>),
    String(String),
    Args(Vec<Expr>),
}

pub(crate) fn parse(ir_line: String, functions: &HashMap<String, Compile>) -> Expr {
    let mut ast = vec![];
    let mut describe = String::new();

    fn default(_expr: Expr) -> io::Result<()> {
        stdout().flush()
    }

    let mut func = Compile {
        f: IRFunc::Void(default),
        requires: vec![],
        c_func: "".to_string(),
    };

    let mut args = vec![];

    let tokens = lexer::Token::lexer(&ir_line);

    for token in tokens {
        let token = get_token(token);

        match token {
            lexer::Token::Descriptor(des) => describe = des,

            lexer::Token::Ident(ident) => {
                if describe.is_empty() {
                    error("Expected descriptor before identifier");
                }

                match describe.as_str() {
                    "func" => {
                        if functions.contains_key(&ident) {
                            let f = functions.get(&ident).unwrap();

                            func = f.clone();
                        }
                    }
                    des => error(format!("Unknown descriptor: {des}").as_str()),
                }
            }

            lexer::Token::String(s) => {
                if describe.is_empty() {
                    error("Expected descriptor before string");
                }

                match describe.as_str() {
                    "arg" => {
                        args.push(Expr::String(s));
                    }
                    des => error(format!("Unknown descriptor: {des}").as_str()),
                }
            }

            lexer::Token::Number(n) => {
                if describe.is_empty() {
                    error("Expected descriptor before number");
                }

                match describe.as_str() {
                    "arg" => {
                        args.push(Expr::Number(n));
                    }
                    des => error(format!("Unknown descriptor: {des}").as_str()),
                }
            }
            _ => error("Expected descriptor token `%` or Identifier"),
        }
    }
    ast.push(Expr::Func(func, Box::new(Expr::Args(args))));

    Expr::Args(ast)
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::ir::setup_functions;

    #[test]
    fn test_parser() {
        let functions = setup_functions();
        let ast = parse("%func print".to_string(), &functions);

        let f = functions.get(&"print".to_string()).unwrap();
        let expected_func = Expr::Func(f.clone(), Box::new(Expr::Args(vec![])));

        assert_eq!(ast, Expr::Args(vec![expected_func]));
    }
}
