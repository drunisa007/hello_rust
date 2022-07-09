


pub fn sum(){
    let mut sum = 0;
    for i in  7..=23{
        sum+=i;
    }

    println!("The sum is {}", sum);
}

pub fn double(){
    let mut count = 0;
    let mut x = 1;

    while x<500{
        count+=1;
        x=x*2;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

pub fn other(args:String){


    let mut count = 0; 
    loop{
      println!("{}",args);
      count+=1;
      if count>7 {
        break;
      }
    }

}
















// pub fn something(){
//     println!("hello from rust");
// }


//ex&cercise
// pub fn print_difference(x: f32, y: f32) {
//     println!("Different between {} and {} is {}", x, y, (x - y).abs());
// }

// pub fn print_array(x: [f64; 2]) {
//     println!("the coordinates are {} {}", x[0], x[1]);
// }

// pub fn check_13(number: i32) -> String{
//     let result = if number == 13 {
//         String::from("it is 13")
//     }
//     else{
//         String::from("not 13")
//     };
//     return result;
// }


// pub fn print_loop(){
//     for i in 0..=50{
//         println!("{}",i);
//     }
// }


// pub fn print_loop(list:[(i32,i32);2]) -> i32{

//     let mut count = 0;

//     for (x,y) in list.iter(){
//         count+=1;
//         println!("x is {} y is {}",x,y);
//     }
    
//     return count;
// }
