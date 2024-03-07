use std::io::Result;

use oort_vm::{compile_ir, emit_and_compile_ir, emit_ir, ir::return_ir_code, run_ir};

fn main() -> Result<()> {
    let ir_ = "%func print %arg \"Hello, world!\"";

    run_ir(ir_.to_string());

    emit_ir(ir_.to_string(), "examples/emit.c")?;

    println!("{}", return_ir_code(ir_.to_string()));

    compile_ir(ir_.to_string(), "examples/emit.out")?;

    emit_and_compile_ir(ir_.to_string(), "examples/emit.c", "examples/emit.out")
}
