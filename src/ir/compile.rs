#[derive(Clone, Debug, PartialEq)]
pub enum IRFunc {
    Normal(fn()),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Compile {
    pub(crate) f: IRFunc,
    pub(crate) requires: Vec<String>,
    pub(crate) ccode: String,
}
