use std::collections::HashMap;
use std::io;
use std::io::{stdout, Write};
use std::sync::Mutex;

use lazy_static::lazy_static;

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

        Mutex::new(m)
    };
}

pub(crate) fn get_from_functions(f: String) -> Option<Compile> {
    let functions = FUNCS.lock().unwrap();
    let get = functions.get(&f);

    get.cloned()
}

pub(crate) fn functions_contains(f: &String) -> bool {
    FUNCS.lock().unwrap().contains_key(f)
}

fn print(args: Expr) -> io::Result<()> {
    match args {
        Expr::_Integer(_) => {}
        Expr::_Add(_, _) => {}
        Expr::_Subtract(_, _) => {}
        Expr::_Multiply(_, _) => {}
        Expr::_Divide(_, _) => {}
        Expr::Func(_, _) => {}
        Expr::String(_) => {}
        Expr::Args(args) => {
            for arg in args {
                match arg {
                    Expr::_Integer(_) => {}
                    Expr::_Add(_, _) => {}
                    Expr::_Subtract(_, _) => {}
                    Expr::_Multiply(_, _) => {}
                    Expr::_Divide(_, _) => {}
                    Expr::Func(_, _) => {}
                    Expr::String(s) => {
                        print!("{}", s);
                    }
                    Expr::Args(_) => {}
                }
            }

            println!();
            return stdout().flush();
        }
    }
    Ok(())
}
