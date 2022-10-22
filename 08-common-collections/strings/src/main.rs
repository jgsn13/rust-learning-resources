#![allow(unused)]

use std::fmt::format;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    /*
        NOTE: strings are pretty dang complicated.
        NOTE: strings are stored as a collection of UTF-8 encoded bytes.
        NOTE: in memory a string is just a list or a collection of ones an zeros.
        NOTE: ASCII is a string encoding, so it defines how to take ones and zeros and turn it into
              a string or take a string and turn it into ones and zeros.
        NOTE: each ASCII character is stored as a byte and only 7 bits of that byte are used to
              represent the character. That means ASCII can only represent 128 unique characters.
              And so, ASCII only represents the english alphabet, some special characters and a few
              commands.
        NOTE: because ASCII only represents english characters other countries created their own
              encoding standards that could represent characters in their languages. This is
              problematic because with all these different encoding standards, how does a program
              know which standard to use when parsing a collection of bytes. And so to solve this
              problem, Unicode was created.
        NOTE: Unicode is a universal character set, meaning that it represents characters from all
              the well-known languages and also things like emojis. Another great part of Unicode
              is that it's backwards compatible with ASCII. And that's because the first 128
              symbols of Unicode are ASCII characters. So you can use a Unicode encoding to parse
              ASCII text.
        NOTE: UTF-8 is a variable with character encoding for Unicode. "variable with" because each
              character in UTF-8 could be represented as one byte, two bytes, three bytes or four
              bytes. This is very important, because in ASCII each character is represented by one
              byte, but with UTF-8 each character could be a different size in terms of bytes.
        NOTE: UTF-8 is the most popular encoding of Unicode and that's why in Rust we use UTF-8 as
              well.
    */

    creating();
    append();
    indexing();
}

fn creating() {
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn append() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!'); // foobar!

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // NOTE: string concatenation requires an owned 'String' on the left.
    // let s3: String = s1 + &s2;
    // println!("{}", s1); // error

    // or we can just use the format! macro (without taking ownership):
    let s3 = format!("{}{}", s1, s2);
}

fn indexing() {
    let hello: String = String::from("Hello");
    // NOTE: we can't index a string by an integer because the length of a string depends on its
    // size and the size of a string character depends on is encoding, so a character like 'й' has
    // two bytez of size.
    // let c: char = hello[0]; // error

    let hello: String = String::from("नमस्ते");

    // Representing the same word:
    // Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    for byte in hello.bytes() {
        println!("{}", byte);
    }

    // Scalar values
    // ['न', 'म', 'स', '्', 'त', 'े'] // in Rust this is what the char type refers to

    for c in hello.chars() {
        println!("{}", c);
    }

    // Grapheme characters
    // ["न", "म", "स्", "ते"]
    // NOTE: in order to keep the Rust standard library clean, the ability to iterate over grapheme
    // clusters is not included by default. To iterate over grapheme clusters we need to import the
    // unicode-segmentation crate.

    for g in hello.graphemes(true) {
        println!("{}", g);
    }

    // NOTE: "a final reason Rust doesn’t allow us to index into a String to get a character is that
    // indexing operations are expected to always take constant time (O(1)). But it isn’t possible
    // to guarantee that performance with a String, because Rust would have to walk through the
    // contents from the beginning to the index to determine how many valid characters there were."
}
