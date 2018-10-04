//-- enum example 1
//fn main() {
//   let home = IpAddr { kind : IpAddrKind::V4,
//                       adr  : String::from("127.0.0.1"),
//                       };
//   let loopback = IpAddr { kind : IpAddrKind::V6,
//                           adr  : String::from("::1"),
//                           };
//   println!("home : {:#?}", home);
//   println!("loopback : {:#?}", loopback);
//}
//#[derive(Debug)]
//enum IpAddrKind {
//     V4,
//     V6,
//}
//#[derive(Debug)]
//struct IpAddr {
//     kind : IpAddrKind,
//     adr : String,
//}

//-- enum example 2
//fn main() {
//   let home = IpAddr::V4(String::from("127.0.0.1"));
//   let loopback = IpAddr::V6(String::from("::1"));
//   println!("home : {:#?}", home);
//   println!("loopback : {:#?}", loopback);
//}
//#[derive(Debug)]
//enum IpAddr{
//     V4(String),
//     V6(String),
//}

//-- enum example 3 : Option
//
//fn main(){
//   let some_number = Some(5);
//   let some_string = Some("hello");
////   let absent_number = None; // --> error, cannt infer type for 'T'
//   let absent_number:Option<i32> = None;
//}


//-- enum example 4 : match expression with enum
//enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
//
//}
//#[derive(Debug)]
//enum UsState {
//     Alabama,
//     Massachusetts,
//     Delaware,
//}
//fn value_in_cents(coin: Coin) -> usize {
//   match coin {
//         Coin::Penny   => {println!("Lucky Penny"); 1 },
//         Coin::Nickel  => 5,
//         Coin::Dime    => 10,
//         Coin::Quarter(state) =>{ println!("State quarter from {:?}!",state); 25 },
//   }
//}
//
//fn main(){
//   let coin1 = Coin::Dime;
//   let coin2 = Coin::Quarter(UsState::Delaware);
//   let coin3 = Coin::Penny;
//
//   let val1 = value_in_cents(coin1);
//   let val2 = value_in_cents(coin2);
//   let val3 = value_in_cents(coin3);
//
//   println!("sum of coins is {}",val1+val2+val3);
//}

//-- enum example 5 : match with Option<T>
//
//fn plus_one(x : Option<i32>) -> Option<i32> {
//   match x {  // match is exhaustive
//         None   => None,
//         Some(i)=> Some(i+1),
//   }      
//}
//
//fn main(){
//   let five = Some(5);
//   let six  = plus_one(five);
//   let none = plus_one(None);
//   println!(" value : {:?}, none {:?}", six, none);
//   
//}

//-- enum example 6 : match with placeholder that is like 'default' of switch in c++
//
//fn main(){
//   let some_u8_value = 1u8;
//
//   match some_u8_value {
//         1 => println!("one"),
//         3 => println!("three"),
//         5 => println!("five"),
//         _ => (),
//   }
//   
//}

//-- enum example 7 : match with 'if let'

fn main(){
   let some_u8_value = Some(0u8);
   //match some_u8_value {
   //      Some(3) => println!("three"),
   //      _ => ()
   //}

//   if let Some(3) = some_u8_value { println!("three"); }

     if let Some(3) = some_u8_value {
        println!("three");
     }
     else {
        println!("others");
     }
}