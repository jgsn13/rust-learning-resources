enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // String
    ChangeColor(i32, i32, i32), // three i32
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKind) {}
