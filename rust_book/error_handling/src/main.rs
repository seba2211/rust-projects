// Chapter 9 

use std::fs::File;
use std::io::ErrorKind;

use std::io;
use std::io::Read;

mod my_lib;  // my lib (as an example)
use my_lib::defs::Guess as Guess;

fn main() {
    
    // panic!("crash and burn");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            },
        }
    };

    /* 
    * A more seasoned Rustacean might write this code instead:
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    */

    // Shortcuts for Panic on Error: unwrap and expect
    let f = File::open("hello.txt").unwrap();  // if this fails, calls panic

    let f = File::open("hello.txt").expect("Failed to open hello.txt");  // if this fails, calls panic with a message

    /* The ? Operator Can Be Used in Functions That Return Result
    * Look at the implementations of 'read_username_from_file' below
    * The functions should return `Result` or `Option` to accept `?`
    */

    /* 
    * The main function is special... The nezt main function compiles and, without errors, opens the files in the variable f
    fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
    */

    // 9.3
    println!("Panic! or not to Panic! section");
    let mut var = 10;
    let mut g1 : Guess = Guess::new(var);
    g1.value_print();

    var = 30;
    g1.modify_value(var);
    g1.value_print();
    
    var = 101;
    g1.modify_value(var);
    g1.value_print();

}

fn read_username_from_file_old1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_old2(filename : &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_old3(filename : &str) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(filename)?.read_to_string(&mut s)?;

    Ok(s)
}

use std::fs;

fn read_username_from_file(filename : &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}


