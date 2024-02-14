use std::{fs::write, io::Result, path::Path};

pub(crate) mod access;
pub(crate) mod compile;
pub(crate) mod lexer;
pub(crate) mod parser;

pub fn run_ir(ir: String) {
    for line in ir.lines() {
        let ast = parser::parse(line.to_string());

        match ast {
            parser::Expr::Args(expr) => {
                for expr in expr {
                    match expr {
                        parser::Expr::Func(f, _args) => match f.f {
                            compile::IRFunc::Normal(f) => f(),
                        },
                        _ => todo!(),
                    }
                }
            }
            _ => todo!(),
        }
    }
}

pub fn emit_ir(ir: String, path: &str) -> Result<()> {
    let mut c = r#"
	int main() {

	"#
    .trim()
    .to_string();

    c.push('\n');

    for line in ir.lines() {
        let ast = parser::parse(line.to_string());

        match ast {
            parser::Expr::Args(expr) => {
                for expr in expr {
                    match expr {
                        parser::Expr::Func(f, _args) => {
                            let mut require = String::new();

                            for req in f.requires {
                                require.push_str("#include<");
                                require.push_str(&req);
                                require.push('>');
                                require.push('\n');
                            }

                            c = format!("{}\n{}", require, c);
                            c.push_str(&f.cfunc);
                            c.push('\n');
                        }
                        _ => todo!(),
                    }
                }
            }
            _ => todo!(),
        }
    }

    c.push('}');

    let path = Path::new(path);

    write(path, c)
}
