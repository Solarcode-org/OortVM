use logos::Logos;

use crate::error::error;
use crate::error::get_token;

use super::access;
use super::compile;
use super::lexer;

pub(crate) enum Expr {
    Integer(i64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Func(compile::Compile, Box<Expr>),
    Args(Vec<Expr>),
}

pub(crate) fn parse(ir_line: String) -> Expr {
    let mut ast = vec![];
    let mut describe = String::new();

    let tokens = lexer::Token::lexer(&ir_line);

    // for word in ir_line.split_whitespace() {

    //     if describe.is_empty() && word.starts_with('%') {
    //         describe = word[1..].to_string();
    //     } else {
    //         match describe.as_str() {
    //             "func" => {
    //                 if access::funcs_contains(word.to_string()) {
    //                     let f = access::funcs_get(word.to_string()).unwrap();

    //                     ast.push(Expr::Func(f, Box::new(Expr::Args(vec![]))))
    //                 }
    //             }
    //             d => {
    //                 panic!("Unknown descriptor: {d}")
    //             }
    //         }
    //     }
    // }

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
