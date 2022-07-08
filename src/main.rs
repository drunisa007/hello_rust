const TOTAL_MISSLE:i32 = 8;
const TOTAL_READY:i32 = 2;

fn main(){
   let mut missle:i32 = TOTAL_MISSLE;
   let ready:i32  = TOTAL_READY;

   missle = missle - ready;


   println!("Shot {} missles and remain {} missles", ready,missle);
}