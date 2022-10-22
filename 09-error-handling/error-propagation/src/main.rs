#![allow(unused)]
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::io::Read;

fn read_usename_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Refactoring the read_usename_from_file function with the '?' operator
fn read_usename_from_file2() -> Result<String, io::Error> {
    // NOTE: with the '?' operator if we fail to get the file, then instead of panicing our
    // function will end early and return the error.
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

// Finally, we can simple chain those method calls
fn read_usename_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Or we can just use the built-in function of fs module
fn read_usename_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// NOTE: we can't use the '?' operator in main function because it can only be used in a
// function that returns 'Result' oe 'Option' (or another type that implements 'Try'). In main
// function the trait 'Try' is implemented for '()' required by 'from_error'.
// let f = File::open("hello.txt")?; // error in main function

// NOTE: Box<dyn Error> is a trait object, which means any type of error.
fn main() -> Result<(), Box<dyn Error>> {
    // NOTE: the main function can also return a Result type.
    let f = File::open("hello.txt")?;

    Ok(())
}
