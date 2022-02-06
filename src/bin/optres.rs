use anyhow::{bail, Result};

fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        bail!("cannot divide by zero")
    }
    Ok(a / b)
}

fn main() {
    divide(4, 2).unwrap();
    divide(4, 0).unwrap();
}
