use std::fs;
use std::error::Error;

pub struct Config{
    pub query : String,
    pub filename : String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argument");
        }
        Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
            }
        )
    }
}

pub fn run (config: Config)->Result<(), Box<dyn Error>>{
    //let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    let contents = fs::read_to_string(config.filename) ? ;
    let rslt = search( &config.query, &contents);
    for item in rslt {
        println!("{}", item);
    }
    //println!("With text:\n{}",contents);
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut List  = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            List.push( line);
        }
    }
    List
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }
}