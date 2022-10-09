fn main() {
    // Rust has a trait, a simple type stored on the Stack, such as integers, booleans and
    // characters implement. This trait allow these types to be copied instead of moved.
    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)

    // s1 was moved, so we cannot use s1 here
    // println!("{}, world!", s1)

    // Rust default is to move a value, and if we want to perform a more specific clone operation
    // there's one method for that:
    let s3 = s2.clone();
    println!("{}, world!", s2);
}
