
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs;

fn main() {
    //-- example 1
    //panic!("crash and burn");

    //-- example 2
    //let v = vec![1,2,3];
    //v[99];

    //-- example 3
/*  let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) =>
        {
            panic!("There was a problem opening the file: {:?}",error)
        },
    };
*/
    //-- example 4
/*    let fname = "hello.txt";
    let f = File::open( fname);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind()
            {
                ErrorKind::NotFound => match File::create(fname){
                    Ok(fc) => fc,
                    Err(e) => panic!("Tried to create file but there was a problem: {:?}",error),
                },
                other_error => panic!("There was a problem opening the file: {:?}",error),
            },
    };
*/
    //-- example 5
    //let f = File::open("hello.txt").unwrap();
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");

    //-- example 6
/*    fn read_username_from_file() -> Result<String, io::Error >{
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e)   => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s){
            Ok(_)  => Ok(s),
            Err(e) => Err(e),
        }
    }
*/
    //-- example 7
/*    fn read_username_from_file() -> Result<String, io::Error>{
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
*/
    //-- example 8
/*    fn read_username_from_file() -> Result<String, io::Error>{
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
*/
    //-- example 9
/*
    fn read_username_from_file() -> Result<String, io::Error>{
        fs::read_to_string("hello.txt")
    }
    let username = match read_username_from_file(){
        Ok(name) => name,
        Err(e  ) => "John".to_string(),
    };
    println!("{}", username);
*/

    //-- example 10
    // error occurred
    let f = File::open("hello.txt")?;

}
