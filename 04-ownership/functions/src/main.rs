fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // error

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s2 = {}", s2); // error
    println!("s3 = {}", s3);
}

fn takes_ownership(some_string: String) {
    // Takes ownership because String type does not implement the Copy trait
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
