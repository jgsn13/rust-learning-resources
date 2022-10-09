fn main() {
    // use the 'mut' keyword to create a muttable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants (uppercase convention) must be type annotated
    const HUMAN_AGE: u32 = 200_00;

    // Shadowing
    // Here we preserve mutability and we can change the type
    let y = 5;
    println!("The value of y is: {}", y);
    let y = 6;
    println!("The value of y is: {}", y);
    let y = "six";
    println!("The value of y is: {}", y);
}
