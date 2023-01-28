#![allow(unused)]
// The static lifetime means that the reference could live as long as the duration of the program.
// All string literals have static lifetime because they are stored in the program's binary.
fn main() {
    let s: &'static str = "I have a static lifetime";
}
