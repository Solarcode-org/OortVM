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

use std::process::exit;
use std::{
    ffi::OsStr,
    fs::write,
    io::{Result, Write},
    path::Path,
    process::Command,
};

use c_emit::{CArg, Code};
use tempfile::NamedTempFile;

use crate::ir::parser::Expr;

use self::access::setup_functions;

pub(crate) mod access;
pub(crate) mod compile;
pub(crate) mod lexer;
pub(crate) mod parser;

/// # Run the IR
///
/// ## Example
///
/// ```rust
/// use oort_vm::run_ir;
///
/// fn main() {
///     let ir_ = "%func print %arg \"Hello, world!\"";
///
///     run_ir(ir_.to_string());
/// }
/// ```
pub fn run_ir(ir: String) {
    let functions = setup_functions();

    for line in ir.lines() {
        let ast = parser::parse(line.to_string(), &functions);

        match ast {
            Expr::Args(expr) => {
                for expr in expr {
                    match expr {
                        Expr::Func(f, args) => match f.f {
                            compile::IRFunc::Void(f) => match f(*args) {
                                Ok(_) => {}
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

/// # Return the C equivalent of the IR.
///
/// ## Example
///
/// ```rust
/// use oort_vm::ir::return_ir_code;
///
/// fn main() {
///     let ir_ = "%func print %arg \"Hello, world!\"";
///
///     println!("{}", return_ir_code(ir_.to_string()));
/// }
/// ```
pub fn return_ir_code(ir: String) -> String {
    let mut c = Code::new();
    let mut requires = vec![];
    let functions = setup_functions();

    for line in ir.lines() {
        let ast = parser::parse(line.to_string(), &functions);

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
                                Expr::Args(args_2) => {
                                    for arg in args_2 {
                                        match arg {
                                            Expr::String(s) => {
                                                args.push(CArg::String(s));
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
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
///     emit_ir(ir_.to_string(), "emit.c")
/// }
/// ```
pub fn emit_ir(ir: String, path: &str) -> Result<()> {
    let c = return_ir_code(ir);

    let path = Path::new(path);

    write(path, c)
}

fn compile_c<S: AsRef<OsStr>>(c_path: S, out_path: &str) -> Result<()> {
    Command::new("gcc")
        .arg(c_path)
        .args(["-o", out_path])
        .output()?;

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
///     compile_ir(ir_.to_string(), "emit.out")
/// }
/// ```
pub fn compile_ir(ir: String, path: &str) -> Result<()> {
    let c = return_ir_code(ir);
    let mut file = NamedTempFile::new()?;

    writeln!(file, "{c}")?;

    compile_c(file.path(), path)?;

    Ok(())
}

#[doc = "# Emit the C equivalent of the IR to a file and compile the IR to machine code and write the
binary to a file."]
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
///     emit_and_compile_ir(ir_.to_string(), "emit.c", "emit.out")
/// }
/// ```
pub fn emit_and_compile_ir(ir: String, c_path: &str, out_path: &str) -> Result<()> {
    let c = return_ir_code(ir);

    let path = Path::new(c_path);
    write(path, c)?;

    compile_c(path, out_path)
}
