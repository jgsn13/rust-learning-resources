#![allow(unused)]
// Generic lifetime annotations describe the relationship between the lifetimes of multiple
// references and how they relate to each other.

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// we say that 'x', 'y' and the return value have the same lifetime because they're related, but
// the lifetime of the return value will be the same lifetime of the parameter that has the
// smallest lifetime.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");

        // the lifetime of 'result' will be the same lifetime as 'string2'
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// Thinking in terms of lifetime, the following code is valid:

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn main2() {
    let string1 = String::from("abcd");
    let result: &str;
    {
        let string2 = String::from("xyz");
        result = longest2(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
