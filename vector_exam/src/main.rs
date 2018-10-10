fn main() {
//   let v: Vec<i32> = Vec::new();
     
     let mut v = vec![1,2,3,4];
     v.push(5);
     v.push(6);
     v.push(7);

// example 1 : accessing an element
//     let third_elem1 = &v[2];
//     let third_elem2 = v.get(2).unwrap();
//     println!("third element way1 {} , way2 {}",third_elem1, third_elem2);

// example 2 : accessing an element out of bounds
//     let index_outof2 = match v.get(100) {
//         Some(_) => println!(" some value {}", v.get(100).unwrap()),
//         None    => println!(" None "),
//     };
//     let index_outof1 = &v[100]; // cause panic

// example 3 : reference borrowing
//   let first = &v[0];
//   v.push(8);  //=> borrow error

// example 4 : change elements
//     for i in &v {
//         println!("{}",i);
//     }
//
//     for i in &mut v {
//         *i += 100;
//     }
//     for i in &v {
//         println!("{}",i);
//     }

// example 5 : multi type
      #[derive(Debug)]
      enum multi_type{
           int(i32),
           float(f64),
           string(String)
      }
      let data = vec![
          multi_type::int(3),
          multi_type::float(0.1235),
          multi_type::string(String::from("bicycle"))
          ];
      for i in &data {
          println!("{:?}", i);
      }
}
