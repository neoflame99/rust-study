use std::clone::Clone;
use std::mem;

fn main(){

  let mut s = vec![4i32; 10];
//let mut b = jmat::Mat {
//    data : s,
//    row : 1,
//    col : 10,
//    nch : 1,
//    tlen: 10,
//    rclen:10,
//} ;
  let mut b : jmat::Mat<i32>= jmat::Mat::new_i32(1,10,1);
  let mut c = b.get_data();
  for i in 0..c.len(){
      c[i] = (i+1) as i32;
  }
    println!("{:?}", b);
//  println!("{:?}", s);

    let mut bx = Box::new([3i32;10]);
    //let mut d = Box::into_raw(bx);
    let mut d = bx.as_mut();
    for i in 0..10{
        d[i]=(i+10) as i32;
    }
    println!("{:?}",bx);
    println!("size of bx {}",mem::size_of_val(&bx));
}