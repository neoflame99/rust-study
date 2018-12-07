//-- lifetime for function
/*
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str{
    if str1.len() > str2.len(){
        str1
    }else{
        str2
    }
}
*/

//-- lifetime for structure
/*
struct ImportantExcerpt<'a>{
    part: &'a str
    //part: &str,
}
fn main(){
    let novel = "Call me Ishmael. Some years age...".to_string();
    let first_sentance = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part : first_sentance};
}
*/

//-- lifetime for a method of a structure
struct ImportantExcerpt<'a>{
    part: &'a str
}
impl<'a> ImportantExcerpt<'a>{
    fn level(&self) -> i32{ 3 }
    fn announce_and_return_part(&self, announcement: &str) -> &str{
        println!("Attention please: {}", announcement);
        self.part
    }
}
fn main(){
    let novel = "Call me Ishmael. Some years age...".to_string();
    let first_sentance = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part : first_sentance};
    println!("{}",i.announce_and_return_part("My name is jb"));
}
