/*
* Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. 
* Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.
*/

fn main() {
    println!("Hello, world! -> lifetimes");

    let first_str = String::from("This is the longest string... I'm much loooonger");
    let second_str = String::from("I'm the shortest string");

    let longest_str = longest(&first_str, &second_str);

    println!("This is the longest str: {}", longest_str);

    let custom_reference = longest_str.split_whitespace().next().unwrap_or("No whitespace found!");

    let mut has_a_ref_inside = HasAReferenceInside{reference: custom_reference};

    println!("My internal reference is: {}", has_a_ref_inside.reference);

    let internal_reference = has_a_ref_inside.announce_and_return_reference(&String::from("Is this an announcement??"));
    println!("Here is again the internal reference: {}", internal_reference);

    // The Static Lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("This variable lives all the code long: {}", s);

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    let announcement_generic = 29234; 
    let shortest_str = shortest_announcement(&first_str, &second_str, announcement_generic, &mut has_a_ref_inside);
    println!("This should be the shortest string: {}", shortest_str);
}

/// returns the longest string between 2 possible strings slices
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {  // the function should specify the lifetime parameter, i.e, 'a
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

/// This is an example struct that holds a &str reference inside
struct HasAReferenceInside<'a> {
    /// The only value that this holds, a reference with a lifetime parameter (a must!)
    reference: &'a str,  // an struct should specify the lifetime parameter, i.e, 'a
}

/// This in an implementation example using lifetimes
/// 
use std::fmt::Display; 
impl<'a>  HasAReferenceInside<'a> {
    fn announce_and_return_reference< T: Display>(&self, announcement: T) -> &str {
        println!("Announcement: {}", announcement);
        self.reference
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together example: longest + announcement

fn shortest_announcement<'a, T>(str1: &'a str, str2: &'a str, announcement: T, smth_has_a_reference: &'a mut HasAReferenceInside<'a>) -> &'a str
where T: Display,
{
    println!("This is an example of Generic Type Parameters, Trait Bounds, and Lifetimes Together");
    if longest(str1, str2) == str1 {
        smth_has_a_reference.reference = str2;
    } else {
        smth_has_a_reference.reference = str1;
    }
    println!("The internal reference of the object is now the shortest string!"); 
    println!("The announcement should implement the Display trait and could be printed!!!");
    smth_has_a_reference.announce_and_return_reference(announcement)  // return the internal reference
}