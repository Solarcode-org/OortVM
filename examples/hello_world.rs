use std::io::Result;

use oort_vm::{compile_ir, emit_and_compile_ir, emit_ir, ir::return_ir_code, run_ir};

fn main() -> Result<()> {
    let ir = "%func print %arg \"Hello, world!\"";

    run_ir(ir);
    separate();

    emit_ir(ir, "examples/emit.c")?;

    println!("{}", return_ir_code(ir));

    compile_ir(ir, "examples/emit.out")?;

    emit_and_compile_ir(ir, "examples/emit.c", "examples/emit.out")
}

fn separate() {
    println!("------");
}
