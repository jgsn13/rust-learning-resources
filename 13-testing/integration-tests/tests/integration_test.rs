use test_example;

mod common;

// run: cargo test --test integration_test
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_example::add_two(2))
}
