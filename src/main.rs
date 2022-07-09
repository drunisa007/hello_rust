// const TOTAL_MISSLE:i32 = 8;
// const TOTAL_READY:i32 = 2;

// #![allow(unused_variables)]


//use  hello::{print_array,print_difference,check_13};


//use hello::{print_loop};


use std::env::{args};

use hello::{sum,double,other};


fn main(){
   


   let args:Vec<String> = args().skip(1).collect();

   //println!("{:?}",args);


   for arg in args{
      if arg=="sum"{
       sum();
      }
      else if arg=="double"{
       double();
      }
      else{
        other();
      }
   }


   //print_loop();
   // let list_array = [(32,342),(23,2)];

   // println!("{}",print_loop(list_array));

   //excercise
   // print_difference(12.6, 2.0);

   // let tuple = ([23,33],[2.1,2.0],(13,22,34));

   // print_array([tuple.2.0 as f64,tuple.2.1 as f64]);


   // println!("{} data ", check_13(tuple.2.0));
   


   //excecise2
   // let width = 4;
   // let height = 7;
   // let depth = 10;


   // let area = area_of(width, height);

   // println!("area is {} !",area);


   // let volume = volume_of(width, height, depth);

   // println!("volume is {}",volume);


   // fn volume_of(width: i32,height: i32,depth:i32) -> i32 {
   //    width*height*depth
   // }


   // fn area_of(width: i32,height: i32) -> i32{
   //    width*height
   // }





   //exercise one

   // let mut missle:i32 = TOTAL_MISSLE;
   // let ready:i32  = TOTAL_READY;

   // missle = missle - ready;


   // println!("Shot {} missles and remain {} missles", ready,missle);
}