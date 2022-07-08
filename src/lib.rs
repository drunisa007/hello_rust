// pub fn something(){
//     println!("hello from rust");
// }


pub fn print_difference(x:f32,y:f32){
    println!("Different between {} and {} is {}",x,y,(x-y).abs());
}

pub fn print_array(x:[f64;2]){
  println!("the coordinates are {} {}",x[0],x[1]);
}


pub fn check_13(number:i32){
    if number==13{
        println!("yes this is 13 yayyyy");
    }
}