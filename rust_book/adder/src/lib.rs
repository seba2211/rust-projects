// WRITING AUTOMATED TESTS
// 11.1 How To Write Tests

// tests 1 - intro
#[cfg(test)]
mod tests_1 {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    /*
    #[test]
    fn this_test_will_fail() {
        panic!("This test fails");
    }
    */
    use super::*;  // We use a glob here so anything we define in the outer module is available to this tests module, for example the Rectangle struct

    #[test]
    fn larger_can_hold_smaller() {
        let larger : Rectangle = Rectangle{width : 8, length : 7};
        let smaller : Rectangle = Rectangle{width: 7, length: 7};
        assert!(larger.can_hold(&smaller));  // assert boolean
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger : Rectangle = Rectangle{width : 8, length : 7};
        let smaller : Rectangle = Rectangle{width: 7, length: 7};
        assert!(!smaller.can_hold(&larger));  // assert boolean
    }
}

#[derive(Debug)]
struct Rectangle {
    width : u32,
    length : u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return (self.width >= other.width) && (self.length >= other.length);  // change the sign > to < to make the test fail
    }
}

// tests 2 - assert_eq! and assert_ne!

#[cfg(test)]
mod tests_2 {
    use super::*;  // We use a glob here so anything we define in the outer module is available to this tests module, for example the add_two function
    #[test]
    fn it_adds_two() {
        let x = 10;
        assert_eq!(add_two(x), x + 2);
    }
    #[test]
    fn it_modifies_input() {
        let x = 1;
        assert_ne!(smth_different_from_input(x), x);
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2  // modify this to make the test fail
}

pub fn smth_different_from_input(x: i32) -> i32 {
    (x + 1) * 2 + 1
}

// tests 3 - adding custom messages on test fail to debug better

#[cfg(test)]
mod tests_3 {

    use super::*;

    #[test]
    fn greetings_contains_name() {
        let name = "Seba";
        let result = greeting(&name);
        // We can see the value we actually got in the test output, which would help us debug what happened instead of what we were expecting to happen.
        assert!(
            result.contains(&name),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    fn greetings_contains_intentional_bug() {
        let name = "Seba";
        let result = greeting("intentional_bug");
        // We can see the value we actually got in the test output, which would help us debug what happened instead of what we were expecting to happen.
        assert!(
            result.contains(&name),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }
}

pub fn greeting(name: &str) -> String {
    format!("greetings {}!", name)
}

// tests 4 - should panic 

#[cfg(test)]
mod tests_4 {
    use super::*;
    #[test]
    #[should_panic]  // this function should panic, if it panics it's ok (we're testing just that)
    fn greater_than_100() {
        Guess::new_1(101);
    }
}

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new_1(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);  // comment to see the test fail
        }
        Guess {value}
    }
}

// tests 5 - should panic with messages and expected values

#[cfg(test)]
mod tests_5 {
    use super::*;
    #[test]
    #[should_panic(expected = "A different string")]  // if the code panics and the received string is different, the test will be considered as FAILED
    fn greater_than_100_should_panic_expected_string_different() {
        Guess::new_2(101);  // this test should fail since the string is different (and the expected message is not contained in the actual message)
    }

    #[test]
    #[should_panic(expected = "Guess value should be less than (or equal) 100")]  // if the code panics and the received string is different, the test will be considered as FAILED
    fn greater_than_100_should_panic_ok() {
        Guess::new_3(101);  // this test should be ok
    }

}

impl Guess {
    pub fn new_2(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value should be greater than (or equal) 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value should be less than (or equal) 100, got {}.", value);
        }
        Guess {value}
    }

    pub fn new_3(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value should be greater than (or equal) 1");
        } else if value > 100 {
            panic!("Guess value should be less than (or equal) 100");
        }
        Guess {value}
    }
}

// tests 6 - using result in Tests

#[cfg(test)]
mod tests_6 {
    #[test]
    pub fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            return Ok(());
        }
        Err(String::from("two plus two does not equal to four"))
    }

    pub fn dbg(msg: &str) {
        println!("[mod tests_6]: {}", msg);
    }

    #[test]
    pub fn a_test_that_only_prints() {
        dbg("[a_test_that_only_prints] You will see this message if you use '--show-output'");
    }

    #[test]
    #[ignore]
    pub fn this_test_is_ignored() {
        panic!("If I run, I panic! Am I an ignored test???");
    }
}

// 11.2 Controlling How Tests are Run

/*
* # Running Tests in Parallel or Consecutively
* If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used,
* you can send the --test-threads flag and the number of threads you want to use to the test binary
*
    cargo test -- --test-threads=1

* # Showing Function Output
* If we want to see printed values for passing tests as well, 
* we can tell Rust to also show the output of successful tests at the end with --show-output
*
    cargo test -- --show-output

* # Running a Subset of Tests by Name
* 
    cargo test <test_name>

* Without using <>
* # Ignoring Some Tests Unless Specifically Requested
* After #[test] we add the #[ignore] line to the test we want to exclude
*
*    #[test]
*    #[ignore]
*    fn expensive_test() {
*    // code that takes an hour to run
*    }
*
* If we want to run only the ignored tests, we can use 
* 
    cargo test -- --ignored
*
*/
