pub fn setup() {
    // setup code specific to your library's tests would go here
    println!("This is an empty setup code");
}

/*

* To avoid having 'common' appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs. 
* This is an alternate naming convention that Rust also understands. 
* Naming the file this way tells Rust not to treat the 'common' module as an integration test file. 
* When we move the setup function code into tests/common/mod.rs and delete the tests/common.rs file, the section in the test output will no longer appear.
* Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

* If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file,
* we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement.
* Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

* This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file.
* Using that structure, integration tests can test the library crate with use to make the important functionality available.
* If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.

*/