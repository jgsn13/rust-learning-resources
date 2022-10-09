fn main() {
    some_function(11, 22);
    let sum = return_function(11, 22);
    println!("The sum is: {}", sum);
}

// statements {} perform some action but don't return a value,
// whereas expressions return a value

fn some_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// By default, functions return an empty tuple ()
fn return_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // let sum = x + y;
    // return sum;

    // ommit semicolon to return:
    x + y
}
