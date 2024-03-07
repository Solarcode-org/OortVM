use std::io;
use std::io::{stdout, Write};
use logos::Logos;

use crate::error::error;
use crate::error::get_token;
use crate::ir::compile::{Compile, IRFunc};

use super::access;
use super::lexer;

#[derive(PartialEq, Debug)]
pub(crate) enum Expr {
    _Integer(i64),
    _Add(Box<Expr>, Box<Expr>),
    _Subtract(Box<Expr>, Box<Expr>),
    _Multiply(Box<Expr>, Box<Expr>),
    _Divide(Box<Expr>, Box<Expr>),
    Func(Compile, Box<Expr>),
    String(String),
    Args(Vec<Expr>),
}

pub(crate) fn parse(ir_line: String) -> Expr {
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
                        if access::functions_contains(&ident) {
                            let f = access::get_from_functions(ident).unwrap();

                            func = f;
                        }
                    }
                    "arg/string" => {
                        args.push(Expr::String(ident));
                    }
                    des => error(format!("Unknown descriptor: {des}").as_str()),
                }
            }

            _ => error("Expected descriptor token `%` or Identifier"),
        }
        
    }
    ast.push(Expr::Func(func, Box::new(Expr::Args(args))));
    println!("{:#?}", ast);

    Expr::Args(ast)
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parser() {
        let ast = parse("%func print".to_string());

        let f = access::get_from_functions("print".to_string()).unwrap();
        let expected_func = Expr::Func(f, Box::new(Expr::Args(vec![])));

        assert_eq!(ast, Expr::Args(vec![expected_func]));
    }
}