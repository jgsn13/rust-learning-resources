#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // run: cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
