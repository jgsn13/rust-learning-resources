fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // consider the following case:
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (), // do nothing
    }

    // with if let syntax:
    if let Some(3) = some_value {
        println!("three");
    }
    // NOTE: if let syntax is not exhaustive
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // NOTE: since match is exhaustive, we can use _ placeholder to specify any other value different
    // from what we want.
    match x {
        Some(i) => Some(i + 1),
        None => None,
        // _ => None,
    }
}
