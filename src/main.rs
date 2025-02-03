// // fn main() {

// // // for intergers whether positive or negative we have i8, i16, i32, i64, i128, isize and for decimals/flaot defualt is f64.
// //  let _x:i16 =10;
// //  let _y:i32 =20;
// //  let _z:i64 =30;

// //  print!("x:{}", _z);

// // }


// fn main(){
//     let ax: &str = "Hello";

//     //fixed type, //vectors // it can change the space at runtime ex- as you append the string. 
// }




//-------strings----//


fn main(){
    let greetings: String = String::from("Hola");
    println!("{}", greetings);                      //basic string



    print!("{}", greetings.chars().nth(1000))        //actual fun part  this is not possble is rust...try pattern matching(some if char or none if overflow) later




}