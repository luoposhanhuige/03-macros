use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    // let ret = f3(f2(f1("hello")?)?)?;
    let ret = my_try!(f3(my_try!(f2(my_try!(f1("hello"))))));
    println!("Final result: {}", ret);
    Ok(())
}

fn f1(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f1: {}", s.as_ref()))
}

fn f2(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f2: {}", s.as_ref()))
}

fn f3(s: impl AsRef<str>) -> Result<String> {
    Err(anyhow!("f3: {}", s.as_ref()))
}

// ? operator, how to simulate it?
// The `?` operator in Rust is used to propagate errors. It can be simulated by using the `Result` type and handling errors explicitly.
// For example, instead of using `?`, you can use a match statement or the `unwrap` method to handle the result of an operation that may fail.
// simulate `?` operator with marco_rules
#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return Err(e.into()),
        }
    };
}

// Purpose: It takes an expression ($expr) that evaluates to a Result type and:
// If the result is Ok(val), it extracts the value val.
// If the result is Err(e), it propagates the error by returning it from the current function.
// This mimics the behavior of the ? operator, which propagates errors automatically.

// what $expr:expr means?
// The `$expr:expr` syntax in a macro definition means that the macro expects an expression as input. The `:expr` part is a fragment specifier that tells the macro to match any valid Rust expression.
// for simulating `?` operator, what does $expr match?
// For simulating the `?` operator, `$expr` matches any expression that returns a `Result` type. This allows the macro to handle both successful and error cases, similar to how the `?` operator works in Rust.
