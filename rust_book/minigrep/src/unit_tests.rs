use super::*;

// These tests were written by me, not related to the tutorial

#[test]
#[ignore]
fn config_err() {
    let args : Vec<String> = vec![String::from("this arg is not considered"), String::from("still lacks one")];
    println!("{:?}", args);

    match Config::new(&args) {
        Ok(_) => panic!("Not ok! Only 2 args given"),
        Err(e) => println!("Test Ok: {}", e)
    }
}

#[test]
#[ignore]
fn config_ok() {
    let args : Vec<String> = vec![String::from("this arg is not considered"), String::from("still lacks one"),String::from("now has 3 args")];
    println!("{:?}", args);

    match Config::new(&args) {
        Ok(config_elem) => {
            println!("query = {}, filename = {}", config_elem.query, config_elem.filename);
            
            if config_elem.query != args[1] {
                panic!("Query element is not correct!");
            } else {
                println!("Query element is correct");
            }

            if config_elem.filename != args[2] {
                panic!("Filename element is not correct!");
            } else {
                println!("Filename element is correct");
            }
        },
        Err(e) => panic!("Not good, 3 args were given {}", e)
    }
    println!("Test ok");
}

// Tutorial tests

#[test]
#[ignore]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";  // Note that the backslash after the opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}