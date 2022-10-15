#![allow(unused)]
// use rand::rngs::ThreadRng;
// use rand::CryptoRng;
// use rand::Rng;

use rand::{rngs::ThreadRng, CryptoRng, Rng};

// use std::io;
// use std::io::Write;

use std::io::{self, Write};

// the Glob operator (to bring in all the public items):
use std::env::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
