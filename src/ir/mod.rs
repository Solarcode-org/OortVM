use std::{ffi::OsStr, fs::write, io::{Result, Write}, path::Path, process::Command};

use tempfile::NamedTempFile;

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

pub fn return_ir(ir: String) -> String {
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
                            c.push_str(&f.ccode);
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

    c
}

pub fn emit_ir(ir: String, path: &str) -> Result<()> {
    let c = return_ir(ir);

    let path = Path::new(path);

    write(path, c)
}

fn compile_c<S: AsRef<OsStr>>(cpath: S, opath: &str) -> Result<()> {
    Command::new("gcc").arg(cpath).args(["-o", opath]).output()?;

    Ok(())
}

pub fn compile_ir(ir: String, path: &str) -> Result<()> {
    let c = return_ir(ir);
    let mut file = NamedTempFile::new()?;

    writeln!(file, "{c}")?;

    compile_c(file.path(), path)?;

    Ok(())
}

pub fn emit_and_compile_ir(ir: String, cpath: &str, opath: &str) -> Result<()> {
    let c = return_ir(ir);

    let path = Path::new(cpath);
    write(path, c)?;

    compile_c(path, opath)
}