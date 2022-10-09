fn main() {
    let s1 = String::from("hello");
    let length = calculate_length(&s1); // passing a reference
    println!("The length of '{}' is {}.", s1, length);

    // mutable references
    let mut s2 = String::from("hello");
    change(&mut s2);

    // NOTE: we can't change the value of a variable while it's being used and that's why we can
    // have only one mutable reference (it prevents data races at compile time), otherwise would
    // not have any mechanism to syncronize data access between the pointers. The same principle is
    // valid to immutable and mutable references at the same time. It's ok to have multiple
    // immutable references because the underline data is not gonna change.
    let mut s3 = String::from("hello");
    // let r1 = &mut s3;
    // let r2 = &mut s3; // error
    // println!("{}, {}", r1, r2);

    // NOTE: the scope of a reference start when it is first introduced and ends when it's used for
    // the last time.
    let mut s4 = String::from("hello");
    let r1 = &s4; // scope starts here
    let r2 = &s4;
    println!("{}, {}", r1, r2); // scope ends here
    let r3 = &mut s4; // at this point r1 and r2 are out of the scope
    println!("{}", r3);

    // dangle references
    // NOTE: that's a reference that points to invalid data because in dangle function s value gets
    // dropped.
    let reference_to_nothing = dangle();

    // The Rules of References
    // 1. At any given time, you can have either one mutable reference or any number of immutable
    //    references.
    // 2. References must always be valid.
}

fn calculate_length(s: &String) -> usize {
    // passing references as function parameters is called borrowing
    // references don't take ownership of the value
    let length = s.len(); // the length of a string
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    // NOTE: because s is defined in the scope of this function when this function get executed
    // rust will drop the string. Therefore the reference will be point to invalid memory.
    let s = String::from("hello");
    &s
}
