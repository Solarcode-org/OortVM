use std::collections::HashMap;
use std::io;
use std::io::{stdin, stdout, Write};
use std::sync::Mutex;

use lazy_static::lazy_static;
use crate::error::{cure_poison, error};

use crate::ir::parser::Expr;

use super::compile::{Compile, IRFunc};

lazy_static! {
    static ref FUNCS: Mutex<HashMap<String, Compile>> = {
        let mut m = HashMap::new();

        m.insert(
            "print".to_string(),
            Compile {
                f: IRFunc::Void(print),
                requires: vec!["stdio.h".to_string()],
                c_func: "printf".to_string(),
            },
        );
        m.insert(
            "input".to_string(),
            Compile {
                f: IRFunc::String(input),
                requires: vec!["stdio.h".to_string()],
                c_func: "scanf".to_string(),
            },
        );

        Mutex::new(m)
    };
}

pub(crate) fn get_from_functions<T: ToString>(f: T) -> Option<Compile> {
    let functions = cure_poison(FUNCS.lock());
    let get = functions.get(&f.to_string());

    get.cloned()
}

pub(crate) fn functions_contains<T: ToString>(f: &T) -> bool {
    let functions = cure_poison(FUNCS.lock());
    functions.contains_key(&f.to_string())
}

fn get_args(tokens: Expr) -> Vec<Expr> {
    if let Expr::Args(args) = tokens {
        args
    } else {
        error("Expected arguments");
    }
}

fn expect_args(tokens: Expr, number_of_args: usize, func: &str) -> Vec<Expr> {
    let args = get_args(tokens);

    if args.len() > number_of_args {
        error(format!("{func}: Too many arguments!"));
    }
    if args.len() < number_of_args {
        error(format!("{func}: Not enough arguments!"));
    }

    args
}

fn print(args: Expr) -> io::Result<()> {
    let args = get_args(args);

    for arg in args {
        if let Expr::String(s) = arg {
            print!("{}", s);
        }
    }

    stdout().flush()?;
    println!();

    Ok(())
}

fn input(args: Expr) -> io::Result<String> {
    let prompt = &expect_args(args, 1, "input")[0];
    let mut answer = String::new();

    match prompt {
        Expr::String(s) => {
            print!("{s}");
            stdout().flush()?;

            match stdin().read_line(&mut answer) {
                Ok(_) => {},
                Err(e) => {
                    panic!("{e}");
                }
            }
        }
        t => {
            error(format!("Expected type string, found {t:?}"));
        }
    }

    Ok(answer)
}
