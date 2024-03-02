mod error;
pub mod ir;

pub use ir::emit_ir;
pub use ir::run_ir;
pub use ir::compile_ir;
pub use ir::emit_and_compile_ir;

#[cfg(test)]
mod tests {
    use super::ir::return_ir;

    #[test]
    fn test_return_ir() {
        let c = return_ir("%func print".to_string());

        let expected = r#"
#include<stdio.h>

int main() {
printf("Hello, World!");
}
        "#.trim().to_string();

        assert_eq!(c, expected);
    }
}