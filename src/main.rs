// // // // fn main() {

// // // // // for intergers whether positive or negative we have i8, i16, i32, i64, i128, isize and for decimals/flaot defualt is f64.
// // // //  let _x:i16 =10;
// // // //  let _y:i32 =20;
// // // //  let _z:i64 =30;

// // // //  print!("x:{}", _z);

// // // // }


// // // fn main(){
// // //     let ax: &str = "Hello";

// // //     //fixed type, //vectors // it can change the space at runtime ex- as you append the string. 
// // // }




// // //-------strings----//


// // fn main(){
// //     let greetings: String = String::from("Hola");
// //     println!("{}", greetings);                      //basic string
// //     print!("{}", greetings.chars().nth(1000))        //actual fun part  this is not possble is rust...try pattern matching(some if char or none if overflow) later

// // }



// //-----loops------


// // fn main(){
// //     for i in 0..10 {
// //         println!("{}", i);
// //     }

// // }



// //......iteration....

// fn get_first_word(sentence: String) -> String {
//     let mut ans: String = String::from("");
//     for char in sentence.chars() {
//         if char == ' ' {
//             break;
//         }
//         ans.push_str(char.to_string().as_str());
//     }

//     return ans;
// }

// fn main() {
//     let sentence = String::from("Hello world!");
//     let first_word = get_first_word(sentence);
//     println!("first word is: {}", first_word);
// }
// // so in a nutshell you need to define the return type of the function. 