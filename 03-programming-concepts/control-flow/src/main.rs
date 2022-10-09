fn if_else() {
    let number = 5;

    // the condition must be explicity a boolean
    if number < 10 {
        println!("first condition was true");
    } else if number < 19 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    // with let
    let condition = true;
    let number = if condition { 5 } else { 6 };
}

fn loops() {
    loop {
        println!("again");
        break; // forever if no break is provided
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // adding counter after break will make this loop return counter
            break counter;
        }
    };
}

fn whiles() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!")
}

fn fors() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // without = the last number will be exclusive
    for number in 0..=4 {
        println!("{}!", number);
    }
}

fn main() {
    if_else();
    loops();
    whiles();
    fors();
}
