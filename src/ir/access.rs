use std::collections::HashMap;
use std::io;
use std::io::{stdout, Write};

use crate::error::error;
use crate::ir::parser::Expr;

use super::compile::{Compile, IRFunc};

pub(crate) fn setup_functions() -> HashMap<String, Compile> {
    let mut m = HashMap::new();

    m.insert(
        "print".to_string(),
        Compile {
            f: IRFunc::Void(print),
            requires: vec!["stdio.h".to_string()],
            c_func: "printf".to_string(),
        },
    );

    m
}

fn to_string(arg: Expr) -> Option<String> {
    match arg {
        Expr::Number(n) => Some(n.to_string()),
        Expr::Func(f, _) => {
            let f = match f.f {
                IRFunc::Void(f) => f,
            };

            Some(format!("{:#?}", f))
        }
        Expr::String(s) => Some(format!("{}", s)),
        _ => None,
    }
}

fn get_args(args: Expr) -> Vec<Expr> {
    match args {
        Expr::Args(args) => args,
        _ => error("Expected arguments"),
    }
}

fn print(args: Expr) -> io::Result<()> {
    let args = get_args(args);

    for arg in args {
        let arg = match to_string(arg) {
            Some(arg) => arg,
            None => error("Data could not be converted to string"),
        };

        print!("{}", arg)
    }

    println!();
    return stdout().flush();
}
