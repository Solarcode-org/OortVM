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