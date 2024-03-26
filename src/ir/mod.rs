//! # The IR Module for Orion.
//!
//! ## Example
//!
//! ```rust
//! use std::io::Result;
//!
//! use oort_vm::{compile_ir, emit_and_compile_ir, emit_ir, ir::return_ir_code, run_ir};
//!
//! fn main() -> Result<()> {
//!     let ir_ = "%func print %arg \"Hello, world!\"";
//!
//!     run_ir(ir_.to_string());
//!
//!     emit_ir(ir_.to_string(), "examples/emit.c")?;
//!
//!     println!("{}", return_ir_code(ir_.to_string()));
//!
//!     compile_ir(ir_.to_string(), "examples/emit.out")?;
//!
//!     emit_and_compile_ir(ir_.to_string(), "examples/emit.c", "examples/emit.out")
//! }
//! ```


use std::{ffi::OsStr, fs::write, io::{Result, Write}, path::Path, process::Command};
use std::process::exit;

use c_emit::{CArg, Code};
use tempfile::NamedTempFile;

use crate::ir::parser::Expr;

pub(crate) mod access;
pub(crate) mod compile;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod construct;

/// # Run the IR
///
/// ## Example
///
/// ```rust
/// use oort_vm::run_ir;
///
/// let ir_ = "%func print %arg \"Hello, world!\"";
///
/// run_ir(ir_.to_string());
/// ```
pub fn run_ir<T: ToString>(ir: T) {
    for line in ir.to_string().lines() {
        let ast = parser::parse(line);

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
                            compile::IRFunc::String(f) => match f(*args) {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("{e}");
                                    exit(1);
                                }
                            }
                        },
                        _ => todo!(),
                    }
                }
            }
            _ => todo!(),
        }
    }
}

/// # Return the C equivalent of the IR.
///
/// ## Example
///
/// ```rust
/// use oort_vm::ir::return_ir_code;
///
/// let ir_ = "%func print %arg \"Hello, world!\"";
///
/// println!("{}", return_ir_code(ir_.to_string()));
/// ```
pub fn return_ir_code<T: ToString>(ir: T) -> String {
    let mut c = Code::new();
    let mut requires = vec![];

    for line in ir.to_string().lines() {
        let ast = parser::parse(line);

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

                            if let Expr::Args(args_2) = *args_ {
                                for arg in args_2 {
                                    if let Expr::String(s) = arg {
                                        args.push(CArg::String(s));
                                    }
                                }
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

/// # Emit the C equivalent of the IR to a file.
///
/// ## Example
///
/// ```rust
/// use std::io::Result;
///
/// use oort_vm::emit_ir;
///
/// fn main() -> Result<()> {
///     let ir_ = "%func print %arg \"Hello, world!\"";
///
///     emit_ir(ir_.to_string(), "examples/emit.c")
/// }
/// ```
pub fn emit_ir<T: ToString, P: AsRef<str>>(ir: T, path: P) -> Result<()> {
    let c = return_ir_code(ir.to_string());

    let path = Path::new(path.as_ref());

    write(path, c)
}

fn compile_c<S: AsRef<OsStr>, O: AsRef<str>>(c_path: S, out_path: O) -> Result<()> {
    Command::new("gcc").arg(c_path.as_ref()).args(["-o", out_path.as_ref()]).output()?;

    Ok(())
}

/// # Compile the IR to machine code and write the binary to a file.
///
/// ## Example
///
/// ```rust
/// use std::io::Result;
///
/// use oort_vm::compile_ir;
///
/// fn main() -> Result<()> {
///     let ir_ = "%func print %arg \"Hello, world!\"";
///
///     compile_ir(ir_.to_string(), "examples/emit.out")
/// }
/// ```
pub fn compile_ir<T: ToString, P: AsRef<str>>(ir: T, path: P) -> Result<()> {
    let c = return_ir_code(ir.to_string());
    let mut file = NamedTempFile::new()?;

    writeln!(file, "{c}")?;

    compile_c(file.path(), path.as_ref())?;

    Ok(())
}

#[doc = "# Emit the C equivalent of the IR to a file and compile the IR to machine code and write
the binary to a file."]
///
/// ## Example
///
/// ```rust
/// use std::io::Result;
///
/// use oort_vm::emit_and_compile_ir;
///
/// fn main() -> Result<()> {
///     let ir_ = "%func print %arg \"Hello, world!\"";
///
///     emit_and_compile_ir(ir_.to_string(), "examples/emit.c", "examples/emit.out")
/// }
/// ```
pub fn emit_and_compile_ir<T: ToString, C: AsRef<str>, O: AsRef<str>>(ir: T, c_path: C,
                                                                      out_path: O) -> Result<()> {
    let c = return_ir_code(ir.to_string());

    let path = Path::new(c_path.as_ref());
    write(path, c)?;

    compile_c(path, out_path.as_ref())
}
