// Scalar (single value) and Compound (group of values) data types

fn scalar() {
    // Integers (signed (ibit) or unsigned (ubit)) (i,u)8,16,23,64,128,size
    let a: i8 = 19; // Decimal
    let b: isize = 19; // Decimal
    let c = 100_000; // Decimal
    let d = 0xff; // Hex
    let e = 0o77; // Octal
    let f = 0b1111_0000; // Binary
    let g = b'A'; // Byte (u8 only)
    let h: u8 = 255; // if h > 255 rust will perform the 2's complement wrapping

    // Floating-point numbers
    let i = 2.0;
    let j: f32 = 3.0;

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f: bool = false;

    // Character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn compound() {
    // Tuple (fixed size) (elements are immutable by default)
    let tup = ("Joaquim Gregório", 19);
    let (name, age) = tup; // destructuring
    let prev_age = tup.1 - 1;

    // Arrays (fixed length)
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    let byte = [0; 8]; // repeat 0 8 times
}

fn main() {
    scalar();
    compound();
}
