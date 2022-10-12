struct Rectangle {
    width: u32,
    height: u32,
}

// Methods:
impl Rectangle {
    // NOTE: the first argument in a method is always 'self' which is the instance the method is
    // being called on.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // &Self is the same as &Rectangle
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// NOTE: the structs allow us to have multiple implementation blocks
// Associated functions (not methods, but like static methods in OOP):
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    // Using associated functions
    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    // NOTE: the syntax to call a method in a reference or directly is the same because Rust has a
    // feature called AUTOMATICALLY REFERENCING AND DEREFERENCING.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    )
}
