#![allow(unused)]

struct Point<T> {
    x: T,
    y: T,
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    p.x(); // y() method is not available for p
    let p1 = Point { x: 5.0, y: 1.0 };
    p1.y();
}
