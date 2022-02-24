use std::env;
use std::process;

use automntcpy::Config;

fn main() {
    
    // get arguments from user, parse in a configuration element
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Args Error: {}", err);
        process::exit(1);
    });

    // run the function
    if let Err(err) = automntcpy::run(&config) {
        eprintln!("Run Error: {}", err);
        process::exit(1);
    }
    println!("Program ended with success");
}
