use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use super::compile::{Compile, IRFunc};

lazy_static! {
    static ref FUNCS: Mutex<HashMap<String, Compile>> = {
        let mut m = HashMap::new();

        m.insert(
            "print".to_string(),
            Compile {
                f: IRFunc::Normal(print),
                requires: vec!["stdio.h".to_string()],
                cfunc: "printf(\"Hello, World!\");".to_string(),
            },
        );

        Mutex::new(m)
    };
}

pub(crate) fn funcs_get(f: String) -> Option<Compile> {
    let funcs = FUNCS.lock().unwrap();
    let get = funcs.get(&f);

    get.cloned()
}

pub(crate) fn funcs_contains(f: &String) -> bool {
    FUNCS.lock().unwrap().contains_key(f)
}

fn print() {
    println!("Hello, World!");
}
