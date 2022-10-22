#![allow(unused)]
use std::collections::HashMap;

fn main() {
    /*
        NOTE: Hashmaps allow you to store key-value pairs, and those keys and values could be of any
              type. Also it uses a hashing function to determine how place those keys and values in
              memory.
    */
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    // NOTE: we are actually moving the ownership of 'blue' and 'yellow' into the HashMap
    // println!("{}", blue); // error

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    updating();
    update_based_on_old_value();
}

fn updating() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // override

    // if we don't want to override:
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);
}

fn update_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // {"hello": 1, "world": 2, "wonderful": 1}

    println!("{:?}", map);
}
