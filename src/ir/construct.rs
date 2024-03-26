use std::fmt::{Display, Formatter};

/// # The Oort IR Generator Struct.
///
/// ## Example
///
/// ```rust
/// use oort_vm::{IRCode, IRArg};
///
/// let mut code = IRCode::new();
///
/// code.call_func_with_args("print", vec![IRArg::String("Hello, world!".to_string())]);
///
/// assert_eq!(code.to_string(), r#"
/// %func print %arg "Hello, world!"
/// "#.trim_start().to_string());
/// ```
pub struct IRCode {
    ir: String
}

/// # The IR Function Argument.
///
/// ## Example
///
/// ```rust
/// use oort_vm::{IRCode, IRArg};
///
/// let mut code = IRCode::new();
///
/// code.call_func_with_args("print", vec![IRArg::String("Hello, world!".to_string())]);
///
/// assert_eq!(code.to_string(), r#"
/// %func print %arg "Hello, world!"
/// "#.trim_start().to_string());
/// ```
pub enum IRArg {
    /// The Argument of type String.
    String(String)
}

impl Default for IRCode {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for IRCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.ir.strip_suffix('\n').unwrap_or(&self.ir))
    }
}

impl IRCode {
    /// # Create a new [`IRCode`] instance.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use oort_vm::IRCode;
    ///
    /// let code = IRCode::new();
    ///
    /// assert_eq!(code.to_string(), "\n".to_string());
    /// ```
    pub fn new() -> Self {
        Self {
            ir: String::new()
        }
    }
    /// # Call a function WITHOUT arguments.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use oort_vm::IRCode;
    ///
    /// let mut code = IRCode::new();
    ///
    /// code.call_func("print");
    ///
    /// assert_eq!(code.to_string(), r#"
    /// %func print
    /// "#.trim_start().to_string())
    /// ```
    pub fn call_func<T: AsRef<str>>(&mut self, func: T) {
        self.ir.push_str("%func ");
        self.ir.push_str(func.as_ref());
        self.ir.push('\n');
    }
    /// # Call a function WITH arguments.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use oort_vm::{IRCode, IRArg};
    ///
    /// let mut code = IRCode::new();
    ///
    /// code.call_func_with_args("print", vec![IRArg::String("Hello, world!".to_string())]);
    ///
    /// assert_eq!(code.to_string(), r#"
    /// %func print %arg "Hello, world!"
    /// "#.trim_start().to_string());
    /// ```
    pub fn call_func_with_args<T: AsRef<str>>(&mut self, func: T, args: Vec<IRArg>) {
        self.ir.push_str("%func ");
        self.ir.push_str(func.as_ref());
        self.ir.push(' ');

        for arg in args {
            match arg {
                IRArg::String(s) => {
                    self.ir.push_str("%arg \"");
                    self.ir.push_str(&s);
                    self.ir.push('"');
                }
            }
            self.ir.push(' ');
        }

        self.ir = self.ir.strip_suffix(' ').unwrap_or(&self.ir).to_string();

        self.ir.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_new() {
        let ir = IRCode::new();

        assert_eq!(ir.to_string(), "\n".to_string());
    }
    #[test]
    fn test_ir_func() {
        let mut ir = IRCode::new();

        ir.call_func("print");

        assert_eq!(ir.to_string(), "%func print\n".to_string());
    }
    #[test]
    fn test_ir_func_with_args() {
        let mut ir = IRCode::new();

        ir.call_func_with_args("print", vec![IRArg::String("Hello".to_string())]);

        assert_eq!(ir.to_string(), "%func print %arg \"Hello\"\n".to_string());
    }
}