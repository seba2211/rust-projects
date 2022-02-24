use std::error::Error;
use std::fs;
use std::env;

#[cfg(test)]
mod unit_tests;  // make the unit_test.rs file visible, only unit test live here [this was not done by the tutorial]

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new_old(args: &[String]) -> Result<Config, String> {  // this implementation uses 'clone' method
        
        let args_size = args.len();
        if  args_size < 3 {  // at least 2 arguments (query, filename) expected as program args
            return Err(format!("Not enough input arguments. Expected at least 2, Given: {}", args_size - 1));
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();  // returns true only if the environment variable is set
    
        Ok (Config {query, filename, case_sensitive})
    }

    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {  // this implementation uses iterators (env::Args)

        args.next();  // skip the first argument -> name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query argument is not present"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename argument is not present"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();  // returns true only if the environment variable is set

        Ok (Config {query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{  
    // in case of error, the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be
    let contents: String = fs::read_to_string(config.filename)?;

    // println!("With text:\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())  // This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.
}

pub fn search_old<'a>(query: &str, content: &'a str) -> Vec<&'a str> {  // uses vectors instead of iterators
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search <'a>(query: &str, content: &'a str) -> Vec<&'a str> {  // uses iterators
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive_old<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {  // uses iterators
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}