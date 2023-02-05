// Iterators have consume methods (like next and sum) and produce methods (like map)

pub trait Iterator {
    type Item; // assossiated type

    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    // let mut v1_iter = v1.iter_mut(); // mutable references
    // let mut v1_iter = v1.into_iter(); // own values

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// 'consume' method
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// 'produce' method
#[test]
fn increment_one() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }
}
