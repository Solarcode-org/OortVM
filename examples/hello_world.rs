use oort_vm::ir;

fn main() {
    ir::run_ir("%func print".to_string());
    ir::emit_ir("%func print".to_string(), "examples/emit.c").unwrap();
}
