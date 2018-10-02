fn main() {
   let s1 = String::from("String length is :");
   let len = calculate_len(&s1);
   println!("{} {}",s1,len);


   let mut s1 = s1;
   change(&mut s1);
   
   println!("{} ",s1);

   let s2 = &mut s1; // first mutable reference -> ok
   println!("{} ",s2);

   let s3 = &mut s1; // second mutable reference -> error
}

fn calculate_len(s : &String) -> u32{
   s.len() as u32
}

fn change(s :&mut String){
   s.push_str(" melong");
}