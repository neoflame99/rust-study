use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    pub query : String,
    pub filename : String,
    pub case_sensitive: bool,
}
/*
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argument");
        }
        Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive : env::var("CASE_INSENSITIVE").is_err(),
            }
        )
    }
}*/
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next(){
            Some(arg) => arg,
            None      => return Err("Didn't get a query string"),
        };
        let filename = match args.next(){
            Some(arg) => arg,
            None      => return Err("Didn't get a file name"),
        };

        Ok(
            Config {
                query: query,
                filename: filename,
                case_sensitive : env::var("CASE_INSENSITIVE").is_err(),
            }
        )
    }
}


pub fn run (config: Config)->Result<(), Box<dyn Error>>{
    //let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    let contents = fs::read_to_string(config.filename) ? ;
    let rslt = search( &config.query, &contents, config.case_sensitive);
    for item in rslt {
        println!("{}", item);
    }
    //println!("With text:\n{}",contents);
    Ok(())
}
/*
fn search<'a>(query: &str, contents: &'a str, case_sense : bool) -> Vec<&'a str>{
    let mut List  = Vec::new();
    if case_sense {
        for line in contents.lines() {
            if line.contains(query) {
                List.push(line);
            }
        }
    }else {
        for line in contents.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                List.push(line);
            }
        }
    }
    List
} */
fn search<'a>(query: &str, contents: &'a str, case_sense : bool) -> Vec<&'a str>{

    if case_sense {
        return contents.lines().filter(|line| line.contains(query)).collect();
    }else {
        return contents.lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect();
        /*
        let mut List  = Vec::new();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                List.push(line);
            }
        }
        return List;
        */
    }

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents, true));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "Rust:
safe, fast, productive.
Pick three.
Trust me!";
        assert_eq!(vec!["Rust:","Trust me!"], search(query, contents, false));
    }
}