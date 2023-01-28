use std::fmt::Display;

// we can implement a trait on a type that implements another trait
impl<T: Display> ToString for T {
    // ...
}

fn main() {}
