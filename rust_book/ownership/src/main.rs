// From chapter "4.3 'String Slices'"
fn main() {
    println!("Chapter 4.3 - 'String Slices'");

    let s = String::from("hello world");
    /*
    * We can create slices using a range within brackets by specifying [starting_index..ending_index]
    */
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("slice 1: {}, slice 2: {}", hello, world);

    let f_word = first_word(&s);
    println!("First word: {}", f_word);


    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // Other Slices

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    
}
// The type that signifies “string slice” is written as &str
fn first_word(s: &str) -> &str {  // returns a string slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // return the 'first word'
        }
    }

    &s[..]  // return the whole string
}