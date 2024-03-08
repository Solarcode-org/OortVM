//! # oort-vm
//!
//! The Oort Virtual Machine (`oort-vm`) is a virtual machine that converts Oort IR to C (which is
//! further compiled to machine code.)
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

#![deny(missing_docs)]

pub use ir::compile_ir;
pub use ir::emit_and_compile_ir;
pub use ir::emit_ir;
pub use ir::run_ir;

mod error;
pub mod ir;

#[cfg(test)]
mod tests {
    use super::ir::return_ir_code;

    #[test]
    fn test_return_ir() {
        let c = return_ir_code("%func print".to_string());

        let expected = r#"
#include<stdio.h>
int main() {
printf();
return 0;
}
"#.trim_start().to_string();

        assert_eq!(c, expected);
    }
}