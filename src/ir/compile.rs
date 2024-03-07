use std::io;
use crate::ir::parser::Expr;

#[derive(Clone, Debug, PartialEq)]
pub enum IRFunc {
    Void(fn(Expr) -> io::Result<()>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Compile {
    pub(crate) f: IRFunc,
    pub(crate) requires: Vec<String>,
    pub(crate) c_func: String,
}
