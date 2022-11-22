
fn main() {
   let mut v = vec![1, 2, 3, 4, 5];

   let first = v[0];

   v.push(6);
   println!("The first element is: {}", first);


   let mut digits: Vec<i32> = vec![2,16,32];
   for i in &mut digits {
       *i <<= 1;
       *i >>= 1;
       *i = i32::pow(*i, 2);
   }
   for i in &digits {
       println!("{}", i);
   }
}
