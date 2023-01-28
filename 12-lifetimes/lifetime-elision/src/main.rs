#![allow(unused)]
// There are scenarios where the compiler can infer the lifetime annotations, and it does it by
// checking the 3 lifetime elision rules. The lifetimes of the arguments being passed in are called
// input lifetimes and lifetimes of the return values called output lifetimes.

// Lifetime Elision rules:
// 1. Each parameter that is a reference gets its own lifetime parameter;
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output
//    lifetime parameters;
// 3. If there are multiple input lifetime parameters, but one of them is &self out &mut self the
//    lifetime of self is assigned to all output lifetime parameters.
fn main() {}

// fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
