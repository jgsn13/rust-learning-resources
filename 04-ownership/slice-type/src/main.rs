fn main() {
    // NOTE: slices don't take ownership of the underline data, just like references

    let mut s = String::from("hello world");
    let s2 = "hello world";
    // NOTE: rust automativally converts s to &str with &s
    // let word = first_word(&s);
    let word = first_word(s2);
    println!("The first word is: {}", word);

    // Slice type with other types:
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
