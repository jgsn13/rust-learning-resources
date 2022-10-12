enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // String
    ChangeColor(i32, i32, i32), // three i32
}

impl Message {
    fn some_function() {
        println!("Some function");
    }
}

fn main() {
    Message::some_function();
}
