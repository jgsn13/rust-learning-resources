#![allow(unused)]

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// following the third lifetime elision rule:
impl<'a> ImportantExcerpt<'a> {
    // fn return_part(&'a self, announcement: &str) -> &'a str {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find first sentence");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
