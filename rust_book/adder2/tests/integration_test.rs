use adder2;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder2::add_two(2));
}

/*
* We’ve added 'use adder2' at the top of the code, which we didn’t need in the unit tests. 
* The reason is that each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope
* 
* We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. 
* Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test

*
    cargo test --test integration_test

* The latter runs only tests included in the 'integration_test.rs' file

*/