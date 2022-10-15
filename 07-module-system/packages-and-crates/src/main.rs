// Rust follows the convention that if you have main.rs defined in your source directory, then a
// binary crate with the same name as your package will be automatically created and main.rs will
// be the crate root. The crate root is the source file that the rust compiler starts at when
// building your crate. It also makes up the root module of your crate.

// NOTE: in this case, our package have two crates following the main.rs and lib.rs convention.

// NOTE: a package must have at least one crate.
// NOTE: a package could have either zero library crates or one library crate.
// NOTE: a package could have any number of binary crates.
fn main() {
    println!("Hello, world!");
}
