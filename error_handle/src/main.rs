
use std::fs::File;

fn main() {
    //-- example 1
    //panic!("crash and burn");

    //-- example 2
    //let v = vec![1,2,3];
    //v[99];

    //-- example 3
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) =>
        {
            panic!("There was a problem opening the file: {:?}",error)
        },
    };
}
