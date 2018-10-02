fn main() {
   let mut counter =0;

   let result = loop {
       counter +=1;

       if counter==100{
          break counter*2;
       }
   };

   assert_eq!(result, 200);
   println!("result {}",result);


       while counter !=0 {
             counter -=1
       };
   println!("counter {}",counter);

   let a:[i32;5] = [10,20,30,40,50];
   for element in a.iter() {
       println!("element {}",element);
   }

   for number in (1..4).rev(){
       println!("count down:{}",number);
   }

}
