// Basically, a dangling reference is a pointer that references a location in memory that may have
// been given to someone else. The memory may have been freed. However, a pointer to that memory
// persists in the program. In summary, it's a reference that points to invalid data.

fn main() {
    let r: &i32;

    {
        let x = 5;
        r = &x;
    } // 'x' has a lifetime that ends up here

    // at this point 'r' is a dangling reference
    println!("r: {}", r);
}

fn valid_lifetime() {
    let x = 5;

    let r = &x;

    println!("r: {}", r); // the lifetime of 'x' is still valid at this point
}
