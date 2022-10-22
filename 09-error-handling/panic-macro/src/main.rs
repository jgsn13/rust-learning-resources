fn main() {
    // NOTE: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    a();
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22");
    }
}
