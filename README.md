`Multifactorials` is a powerful mathematics tool who allows you to calculate factorials easily.

# Installation

To use `multifactorials`, you need to add it to your dependencies like in the example bellow:

```toml
[package]
name = "my-beautiful-crate"
version = "0.1.0"

[dependencies]
multifactorials = "0.3.0"
```

# Example
```rust
use multifactorials::Multifactorials;

fn main() {
    let simple_factorial = Multifactorials::simple(7.0);
    let complex_factorial = Multifactorials::complex(7.0, 2);
    
    println!("The result of 7! is {}", simple_factorial);
    println!("The result of 7!! is {}", complex_factorial);
}
```