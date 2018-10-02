fn main() {
    let mut user1 = User {
        name : String::from("steve"),
        email: String::from("steve@google.com"),
        sign_in_count: 1,
        active: true,
        };
//    println!("{}",user1);
      user1.email = String::from("steve@daum.net");
    let user2 = User {
        name : String::from("john"),
        email: String::from("john@google.com"),
        ..user1
        };

    let red = Color(255,0,0); // structure like tuple
    let pos = Point(10,24,0);

    let rec = Rectangle{
            width : 30, height : 50,
        };
    let area = cal_area( &rec );
    println!("The area of the rectangle is {}", area);
    println!("The rectangle has {:#?}", rec);

    let area2 = rec.eval_area();
    let area3 = rec.eval_area();
    println!("The area of the rectangle is {}", area2);

    let area4 = Rectangle::area_like_static(&rec);
    println!("The area of the rectangle is {}", area4);

    let rec2 = Rectangle{ width : 10, height : 20 };
    let rec3 = Rectangle{ width : 100, height: 30 };
    println!("Can rec hold rec2 ? : {}", rec.can_hold(&rec2));
    println!("Can rec hold rec3 ? : {}", rec.can_hold(&rec3));

    let square = Rectangle::square(15);
    println!("{:#?}", square);
}
struct User{
       name : String,
       email: String,
       sign_in_count: u64,
       active: bool,
}
struct Color(i32, i32, i32); // structure like tuple
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle{
       width : usize,
       height: usize,
}
impl Rectangle {
     fn eval_area(&self) -> usize {
     //fn eval_area(self) -> usize { // the method takes ownership of self
        self.width * self.height
     }

     fn can_hold(&self, other: &Rectangle) -> bool {
        //if self.width > other.width && self.height > other.height { true }
        //else { false }
        self.width > other.width && self.height > other.height
     }

     //- associated function that is like a static method of c++
     fn area_like_static(rect: &Rectangle) -> usize { // it's like a static method of c++
        rect.width * rect.height
     }
     fn square (size : usize) -> Rectangle {
        Rectangle { width : size, height : size }
     }

}

fn cal_area( rect : &Rectangle) -> usize {
   rect.width * rect.height
}