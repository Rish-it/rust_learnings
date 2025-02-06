// // // // // // // // // fn main() {

// // // // // // // // // // for intergers whether positive or negative we have i8, i16, i32, i64, i128, isize and for decimals/flaot defualt is f64.
// // // // // // // // //  let _x:i16 =10;
// // // // // // // // //  let _y:i32 =20;
// // // // // // // // //  let _z:i64 =30;

// // // // // // // // //  print!("x:{}", _z);

// // // // // // // // // }


// // // // // // // // fn main(){
// // // // // // // //     let ax: &str = "Hello";

// // // // // // // //     //fixed type, //vectors // it can change the space at runtime ex- as you append the string. 
// // // // // // // // }




// // // // // // // //-------strings----//


// // // // // // // fn main(){
// // // // // // //     let greetings: String = String::from("Hola");
// // // // // // //     println!("{}", greetings);                      //basic string
// // // // // // //     print!("{}", greetings.chars().nth(1000))        //actual fun part  this is not possble is rust...try pattern matching(some if char or none if overflow) later

// // // // // // // }



// // // // // // //-----loops------


// // // // // // // fn main(){
// // // // // // //     for i in 0..10 {
// // // // // // //         println!("{}", i);
// // // // // // //     }

// // // // // // // }



// // // // // // //......iteration....

// // // // // // fn get_first_word(sentence: String) -> String {
// // // // // //     let mut ans: String = String::from("");
// // // // // //     for char in sentence.chars() {
// // // // // //         if char == ' ' {
// // // // // //             break;
// // // // // //         }
// // // // // //         ans.push_str(char.to_string().as_str());
// // // // // //     }

// // // // // //     return ans;
// // // // // // }

// // // // // // fn main() {
// // // // // //     let sentence = String::from("Hello world!");
// // // // // //     let first_word = get_first_word(sentence);
// // // // // //     println!("first word is: {}", first_word);
// // // // // // }


// // // // // ######------Rust-Jargons------######

// // // // // <------Mutability------>
// // // // //  fn main(){
// // // // //     stack_fn();
// // // // //     heap_fn();
// // // // //     update_str_fn();
// // // // //  }

// // // // //  fn stack_fn(){
// // // // //     let x=1;
// // // // //     let y=68;
// // // // //     let z=x+y;
// // // // //     println!("stack function:the sum of {} and {} is {}",x,y,z);
// // // // //  }

// // // // //  fn heap_fn(){
// // // // //     let f1=String::from ("hello");
// // // // //     let f2=String::from ("rust");
// // // // //     let f=format!("{} {}", f1, f2);
// // // // //     println!("Heap function: Combined string is '{}'", f);
// // // // //  }


// // // // //  fn update_str_fn(){
// // // // //     let mut f = String:: from ("initial");
// // // // //     println!("capacity:{}, Length:{}, pointer:{:p}", f.capacity(), f.len(), f.as_ptr());

// // // // // for _ in 0..100{
// // // // //     f.push_str(" and some additional text");
// // // // //     println!("capacity:{}, Length:{}, pointer:{:p}", f.capacity(), f.len(), f.as_ptr()); // delibratly tried to change the pointer->
// // // // // }  
// // // // // }



// // // // // <------Ownership (In a nushell automatic deallocation)------>
// // // //   fn main(){
// // // //     let x1= String::from ("why rust!");
// // // //     let x2=x1;     //extremely memory safe...

// // // //     println!("{}", x2)    // error x1 is no longer a bf. new owner is x2.
// // // //   }


// // // fn main(){
// // //   let mut my_string=String::from("hey");
// // //   // takes_ownership(my_string.clone());                //you can use clone to fix below error but its kind a expenisve operation and its not feasible everytime.
// // //   my_string=takes_ownership(my_string);    //takes ownership
// // //   println!("{}", my_string);                           //<--ERROR: my_string is moved 
// // // }

// // // fn takes_ownership(some_stirng:String) -> String{
// // //   println!("{}", some_stirng);
// // //   return some_stirng;                 //----> And she is back don't forget 3 step process..

// // ///---> one more thing if she get back 
// // /// let my_string_3=same
// // /// println(same) ---> what did we learn rehana is a whore...
// // // }


// // //----->>Borrowing anf refrences...  You know like casual stuff

// // fn main(){   
// //     let x1= String::from ("hello");
// //     let x2= &x1;
// //     println!("{}", x2);
// //     println!("{}", x1);             // she is just borrowed for while Let's See what future holds for us.
// // }


// fn main(){
//   let mut x1=String::from("hey");
//   update_str_fn(&mut x1);              //mutable refrence... 
//   println!("{}", x1)
// }

// fn update_str_fn(x:&mut String){            //takes mutable string
//   x.push_str("Product Manager")
// }


//------->>>Let's Make borrower memory safe...

fn main(){
     let mut x1=String::from("hey");
     let x2: &mut String =&mut x1;
     x2.push_str("rust");              //only one mut at a time only x2. Can push no x3 &x1 or &mut x1
     println!("{}", x1)                       //another way to pass mutable refrence...
  }
 
