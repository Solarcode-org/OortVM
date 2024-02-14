#[derive(Clone)]
pub enum IRFunc {
    Normal(fn()),
}

#[derive(Clone)]
pub struct Compile {
    pub(crate) f: IRFunc,
    pub(crate) requires: Vec<String>,
    pub(crate) cfunc: String,
}
