use std::io;

use crate::ir::parser::Expr;

#[derive(Clone, Debug)]
pub enum IRFunc {
    Void(fn(Expr) -> io::Result<()>),
    String(fn(Expr) -> io::Result<String>)
}

impl PartialEq for IRFunc {
    fn eq(&self, other: &Self) -> bool {
        let f = match &self {
            IRFunc::Void(f) => {
                *f as usize
            }
            IRFunc::String(f) => {
                *f as usize
            }
        };
        let other = match other {
            IRFunc::Void(other) => {
                *other as usize
            }
            IRFunc::String(other) => {
                *other as usize
            }
        };

        f == other
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Compile {
    pub(crate) f: IRFunc,
    pub(crate) requires: Vec<String>,
    pub(crate) c_func: String,
}
