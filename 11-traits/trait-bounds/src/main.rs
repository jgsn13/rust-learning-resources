#![allow(unused)]
use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news!: {}", item.summarize());
}

// here we have more flexibility:
pub fn notify1(item1: &(impl Summary + Display), item2: &impl Summary) {
    // ...
}

// here we can ensure that the parameters are of the same type:
pub fn notify2<T: Summary + Display>(item1: &T, item2: &T) {
    // ...
}

// to avoid a lot of text between the function name and its parameters we can use the 'where'
// clause:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    // ...
}

fn some_function2<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The Sky is not actually falling."),
    };

    notify(&article);
}
