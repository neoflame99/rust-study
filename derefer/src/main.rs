/*
fn main() {
    let mut x = 5;
    let mut y = &mut x;

    *y = 10;
    //println!("x={},*y={},y={}",x,*y, y);
    println!("*y={},y={}",*y, y);
    //assert_eq!(5, x);
    //assert_eq!(5, *y);
}
*/
/*
fn main() {
    let mut x = 5;
    let mut y = Box::new(x);

    *y = 10;
    println!("x={},*y={},y={}",x,*y, y);
    //assert_eq!(10, x);
    //assert_eq!(5, *y);
    //assert_eq!(10, y);
}
*/
/*
use std::ops::{Deref, DerefMut};

fn main(){
    let mut x = 5;
    let mut y = MyBox::new(x);
    *y = 10;
    println!("x={},*y={},y={:?}",x,*y, y);
    //assert_eq!(10, x);
    //assert_eq!(5, *y);
    //assert_eq!(10, y);
    let m = MyBox::new("rust".to_string());
    hello(&m);
    hello(&(*m)[..]);
}
fn hello(name: &str){
    println!("hello {}!",name);
}
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> DerefMut for MyBox<T>{

    fn deref_mut(&mut self) -> &mut T{
        &mut self.0
    }
}
impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}
*/

//-- Drop trait example
struct CustomSmartPointer{
    data : String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!",self.data);
    }
}
fn main(){
    let c = CustomSmartPointer { data: String::from("my stuff")};
    let d = CustomSmartPointer { data: String::from("other stuff")};
    println!("CustomSmartPointer created.");
    //c.drop();
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
