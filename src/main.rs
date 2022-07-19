// const TOTAL_MISSLE:i32 = 8;
// const TOTAL_READY:i32 = 2;

// #![al&&low(unused_variables)]


//use  hello::{print_array,pri&nt_difference,check_13};


//use hello::{print_loop};



//use hello::{sum,double,other};

#![allow(dead_code,unused_variables)]


fn main(){



   // first : create simple rectangle
   // second: use method to get area
   // third : use struct to decrease the area



   // let mut my_hash: HashMap<u8,bool> = HashMap::new();

   // my_hash.insert(0, false);
   // my_hash.insert(5, true);

   // let result = my_hash.remove(&6).unwrap();




   // println!("{:?}",result);

//  let mut vect:Vec<i32> = vec![2,2,2];

//  vect.push(4);

//  println!("{:?}",vect);




// excercise 6 
//    trait  Bite {
//        fn bite(self:&mut Self);
//    }

//    #[derive(Debug)]
//    struct Grapes {
//       count:i32
//    }

//    impl Bite for Grapes{
//     fn bite(self:&mut Self) {
//        self.count = self.count-1;
//     }
// }

//   let mut  grap = Grapes{
//    count:32
//   };

//   grap.bite();

//   println!("{:?}",grap);


//   generic_bite(&mut grap);

//   println!("{:?}",grap);


  


//   fn generic_bite<T:Bite>(food: &mut T){
//    food.bite();
//   }
    
  
   //exercise 5

   // let one = 1;
   // let two = 2;



   // println!("1 + 2 = {}, even via references", add(&one, &two));

   // println!("{} {}",one,two);


   // fn add(x1: &i32,x2: &i32)-> i32{
   //   //(*x1)+(*x2)
   //   x1+x2
   // }


//   let mut arg = std::env::args().nth(1).unwrap_or_else(||{
//    println!("Please supply an argument to this program.");
//    std::process::exit(-1);
//   });


//   println!("{:?}",arg);

//   inspect(&arg);

// //    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
// //       println!("Please supply an argument to this program.");
// //       std::process::exit(-1);
// //   });


//   change(&mut arg);

//   println!("{}",arg);


//   if eat(&arg){
//    println!("yes may be banana");
//   }
//   else {
//    println!("no bana");
//   }



//   fn eat(s:&String)-> bool{
//     s.starts_with("b")&&s.contains("a")
//   }


// fn change(s:&mut String){
//    if !s.ends_with("s"){
//       s.push_str("s");
//    }
// }


// fn inspect(s:&String){
//   if s.ends_with("s"){
//    println!("it is pural")
//   }
//   else{
//    println!("it is singular")
//   }
// }


    

   // let mut  s1 = String::from("Hello World");

   // do_staff(&mut s1);

   // println!("{}",s1);

   // fn do_staff(s:&mut String){
   //    s.insert_str(0, "hello");
   //   println!("{}",s);
   // }

   //excecise 4
   // let args:Vec<String>  = std::env::args().skip(1).collect();

   // //println!("{:?}",args);


   // for arg in args{
   //    if arg=="sum"{
   //     sum();
   //    }
   //    else if arg=="double"{
   //     double();
   //    }
   //    else{
   //      other(arg.clone());
   //    }
   // }


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