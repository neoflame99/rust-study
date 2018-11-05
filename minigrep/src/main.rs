
use std::env;
use std::fs;

fn main() {
    let args : Vec<String>= env::args().collect();

    let narg = args.len();
    if narg < 3 {
        panic!("arguments are not enough");
    }
    let config = parse_config(&args);

    println!("{}",narg);
    println!("Searching for {}",config.query);
    println!("in file {}",config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}",contents);
}
struct Config{
    query : String,
    filename : String,
}
fn parse_config(args: &[String]) -> Config{
    Config {
        query : args[1].clone(),
        filename : args[2].clone(),
    }
}