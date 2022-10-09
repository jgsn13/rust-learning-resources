fn main() {
    // ----- Ownership rules -----
    // 1. Each value in Rust has variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // s is not valid here, it's not yet declared
    {
        let s = "hello"; // s is valid from this point foward
                         // do stuff with s
    }
    // this scope is now over, and s is no longer valid

    // Rust automatically allocate memory on the Heap
    {
        let s = String::from("hello");
    }
    // this scope is now over, and s is no longer valid, meaning that Rust deallocates the memory
    // on the Heap
}
