#![allow(unused)]

// NOTE: vectors can only store one type of data

fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // NOTE: if we want to create a vector and initialize it with values at the same time Rust has
    // a convenient macro for that.
    let v2 = vec![1, 2, 3, 4, 5];

    // NOTE: this way of accessing vector data expects to get runtime errors if the index is out of
    // bound.
    let third = &v2[2];
    println!("The third element is {}", third);

    // NOTE: we can use the get() method to access vector data preventing runtime errors.
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // NOTE: the following code give us an error because to push a new element to a vector all its
    // elements must be reallocated, so we cannot have an immutable reference if the vector is
    // mutable. This happens because 'third' is a reference.
    let third = &v2[2];
    // v2.push(6); // error
    println!("The third element is {}", third);

    // Iterate over an vector:
    iterate();

    // Storing different types of data:
    different_types();
}

fn iterate() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        // dereference to add 50
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }
}

fn different_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // match &row[1] {
    //     SpreadsheetCell::Int(i) => println!("{}", i),
    //     _ => println!("Not an integer!"),
    // }

    match row.get(10) {
        Some(SpreadsheetCell::Int(i)) => println!("{}", i),
        Some(_) => println!("Not an integer!"),
        None => println!("Cell not exists!"),
    }
}
