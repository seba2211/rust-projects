// Chapter 12: An I/O Project: Building a Command Line Program

use std::env;
use std::process;

// We add a use minigrep::Config line to bring the Config type from the library crate into the binary crate’s scope,
// and we prefix the run function with our crate name. 
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();  // old implementation uses vectors and the new_OLD method from Config
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    );

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // We use if let rather than unwrap_or_else to check whether run returns an Err value 
    // Because run returns () in the success case, we only care about detecting an error,
    // so we don’t need unwrap_or_else to return the unwrapped value because it would only be ()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


