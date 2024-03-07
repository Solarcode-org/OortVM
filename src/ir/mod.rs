use std::{ffi::OsStr, fs::write, io::{Result, Write}, path::Path, process::Command};
use std::process::exit;

use c_emit::{CArg, Code};
use tempfile::NamedTempFile;

use crate::ir::parser::Expr;

pub(crate) mod access;
pub(crate) mod compile;
pub(crate) mod lexer;
pub(crate) mod parser;

pub fn run_ir(ir: String) {
    for line in ir.lines() {
        let ast = parser::parse(line.to_string());

        match ast {
            Expr::Args(expr) => {
                for expr in expr {
                    match expr {
                        Expr::Func(f, args) => match f.f {
                            compile::IRFunc::Void(f) => match f(*args) {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("{e}");
                                    exit(1);
                                }
                            },
                        },
                        _ => todo!(),
                    }
                }
            }
            _ => todo!(),
        }
    }
}

pub fn return_ir_code(ir: String) -> String {
    let mut c = Code::new();
    let mut requires = vec![];

    for line in ir.lines() {
        let ast = parser::parse(line.to_string());

        match ast {
            Expr::Args(expr) => {
                for expr in expr {
                    match expr {
                        Expr::Func(f, args_) => {
                            for req in f.requires {
                                if !requires.contains(&req) {
                                    requires.push(req.clone());
                                    c.include(&req);
                                }
                            }

                            let mut args = vec![];

                            match *args_ {
                                Expr::_Integer(_) => {}
                                Expr::_Add(_, _) => {}
                                Expr::_Subtract(_, _) => {}
                                Expr::_Multiply(_, _) => {}
                                Expr::_Divide(_, _) => {}
                                Expr::Func(_, _) => {}
                                Expr::Args(args_2) => {
                                    for arg in args_2 {
                                        match arg {
                                            Expr::_Integer(_) => {}
                                            Expr::_Add(_, _) => {}
                                            Expr::_Subtract(_, _) => {}
                                            Expr::_Multiply(_, _) => {}
                                            Expr::_Divide(_, _) => {}
                                            Expr::Func(_, _) => {}
                                            Expr::String(s) => {
                                                args.push(CArg::String(s));
                                            }
                                            Expr::Args(_) => {}
                                        }
                                    }
                                }
                                Expr::String(_) => {}
                            }

                            c.call_func_with_args(&f.c_func, args);
                        }
                        _ => todo!(),
                    }
                }
            }
            _ => todo!(),
        }
    }

    c.to_string()
}

pub fn emit_ir(ir: String, path: &str) -> Result<()> {
    let c = return_ir_code(ir);

    let path = Path::new(path);

    write(path, c)
}

fn compile_c<S: AsRef<OsStr>>(c_path: S, out_path: &str) -> Result<()> {
    Command::new("gcc").arg(c_path).args(["-o", out_path]).output()?;

    Ok(())
}

pub fn compile_ir(ir: String, path: &str) -> Result<()> {
    let c = return_ir_code(ir);
    let mut file = NamedTempFile::new()?;

    writeln!(file, "{c}")?;

    compile_c(file.path(), path)?;

    Ok(())
}

pub fn emit_and_compile_ir(ir: String, c_path: &str, out_path: &str) -> Result<()> {
    let c = return_ir_code(ir);

    let path = Path::new(c_path);
    write(path, c)?;

    compile_c(path, out_path)
}