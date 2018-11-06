extern crate minigrep;

use std::env;
use std::process;
use minigrep::*;

fn main() {
    let args : Vec<String>= env::args().collect();
/*
    let config = match Config::new(&args) {
        Ok (Config) => Config,
        Err(E)     => {println!("{}",E); std::process::exit(1) },
    };
*/
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("in file {}",config.filename);

    //run(config);
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
