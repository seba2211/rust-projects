// From chapter "4.2 References and Borrowing"
fn main() {
    println!("Chapter 4.2 - 'References and Borrowing'");

    // Simple reference (could not be muted!)
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable References

    let mut s = String::from("hello");

    println!("New mutable string: {}", s);

    change(&mut s);

    println!("Mutable string changed: {}", s);

    /*
    * You can have only one mutable reference to a particular piece of data in a particular scope
    * This will fail:
    *   let mut s = String::from("hello");

    *   let r1 = &mut s;
    *   let r2 = &mut s;
    * This will do the trick:
    */
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    /*
    * We also cannot have a mutable reference while we have an immutable one.
    */

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    /*
    * Users of an immutable reference don’t expect the values to suddenly change out from under them!
    * However, this will work because the last usage of the immutable references occurs before the mutable reference is introduced:
    */

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    change(r3);
    println!("{}", r3);

    // DANGLE POINTERS

    // dangle();

    /*
    * The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. 
    * These scopes don’t overlap, so this code is allowed.
    */

    // The Rules of References
    /*
     * - At any given time, you can have either one mutable reference or any number of immutable references.
     * - References must always be valid.
     */

}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ENABLING THIS FUNCTION WON'T MAKE THIS CODE TO COMPILE
/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/