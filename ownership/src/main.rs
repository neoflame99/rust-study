fn main() {
   let x = 5;
   let y = x.clone(); // same as let y = x;

   println!("x, y = {}, {}",x,y);
   
   let mut s1 = String::from("Hello");
   
   {
      let s2 = s1;  // move ownership
      println!("{} ", s2);
      s1 = s2;      // give back ownership
   }

   println!("{} ", s1);

   let s3 = s1.clone(); // cloning or deep copy
   println!("{} ", s1);
   println!("{} ", s3);

   takes_value( x);
   println!("x is {}",x);

   takes_ownership(s1); // ownership is moved
//   println!("{} ", s1); // -> occure error
}

fn takes_ownership(x : String){
   println!("{} ", x);
}

fn takes_value(x : i32){
   println!("x is {} ", x);
}