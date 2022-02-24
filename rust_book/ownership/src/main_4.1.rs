// From chapter "4.1. What is Ownership?"

fn main() {
    
    let mut s = String::from("hello");
    /*
    * A String is made up of three parts: 
    * a pointer to the memory that holds the contents of the string, a length, and a capacity. 
    * This group of data is stored on the stack.
    * The memory on the heap is the one that holds the contents
    */

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // ownership - 1
    let s1 = String::from("hello");
    let s2 = s1;  // after this, s1 becomes invalid

    /*
    * "But because Rust also invalidates the first variable, instead of being called a shallow copy,
    * it’s known as a move. In this example, we would say that s1 was moved into s2"
    */

    println!("{}, world!", s2);  // if s1 is used, this will generate an error!

    // clone - may be expensive
    
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // this is valid! -> stack-only data
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    /*
    * "Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. 
    * That means there’s no reason we would want to prevent x from being valid after we create the variable y"
    */

    // Ownership and Functions

    println!("Ownership and Functions");

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    
    // Return Values and Scope

    println!("Return Values and Scope");

    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    /*
    * The ownership of a variable follows the same pattern every time: assigning a value to another variable **moves** it. 
    */
    println!("Returning a tuple");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
} 

// Ownership and Functions

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


// Return Values and Scope

fn gives_ownership() -> String {             // gives_ownership will move its
                                            // return value into the function
                                            // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                            // moves out to the calling
                                            // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

    a_string  // a_string is returned and moves out to the calling function
}

//

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
