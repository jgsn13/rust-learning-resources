pub fn add_two(a: i32) -> i32 {
    a + 2
}

// run: cargo test tests::
#[cfg(test)]
mod tests {
    use super::*;

    // run: cargo test add

    #[test]
    fn add_two_and_two() {
        // run: cargo test add_two_and_two
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        // run: cargo test add_three_and_two
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        // run: cargo run one_hundred
        assert_eq!(102, add_two(100));
    }
}
