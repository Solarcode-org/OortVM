use std::collections::HashMap;
use std::io;
use std::io::{stdout, Write};
use std::sync::Mutex;

use lazy_static::lazy_static;
use crate::error::cure_poison;

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

pub(crate) fn get_from_functions<T: ToString>(f: T) -> Option<Compile> {
    let functions = cure_poison(FUNCS.lock());
    let get = functions.get(&f.to_string());

    get.cloned()
}

pub(crate) fn functions_contains<T: ToString>(f: &T) -> bool {
    let functions = cure_poison(FUNCS.lock());
    functions.contains_key(&f.to_string())
}

fn print(args: Expr) -> io::Result<()> {
    if let Expr::Args(args) = args {
        for arg in args {
            if let Expr::String(s) = arg {
                print!("{}", s);
            }
        }

        stdout().flush()?;
        println!();
    }
    Ok(())
}
