#![allow(unused)]

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    // cannot capture the envitonment:
    // fn equal_to_x(z: i32) -> bool {
    //     z ==x
    // }

    let y = 4;

    assert!(equal_to_x(y))
}

// NOTE: the three ways to capture the envitonment are encoded in function traits:
// - FnOnce: takes ownership of the variables inside the closures envitonment. The 'Once' part of the
// name represents the fact that closures can't take ownership of the same variables more than
// once, so these closures can only be called once.
// - FnMut: mutably borrow values.
// - Fn: immutably borrow values.

// NOTE: when you create a closure rust infers which of these traits to use based on how you use
// the values inside the closures envitonment.

// NOTE: we could force the closure to take ownership by specifying the 'move' keyword in front of
// the closure definition.
fn force_take_ownership() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y))
}
