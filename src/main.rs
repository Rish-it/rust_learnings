// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // fn main() {

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // for intergers whether positive or negative we have i8, i16, i32, i64, i128, isize and for decimals/flaot defualt is f64.
// // // // // // // // // // // // // // // // // // // // // // // // // // // // // //  let _x:i16 =10;
// // // // // // // // // // // // // // // // // // // // // // // // // // // // // //  let _y:i32 =20;
// // // // // // // // // // // // // // // // // // // // // // // // // // // // // //  let _z:i64 =30;

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // //  print!("x:{}", _z);

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // // // // // // // // // // // // //     let ax: &str = "Hello";

// // // // // // // // // // // // // // // // // // // // // // // // // // // // //     //fixed type, //vectors // it can change the space at runtime ex- as you append the string. 
// // // // // // // // // // // // // // // // // // // // // // // // // // // // // }




// // // // // // // // // // // // // // // // // // // // // // // // // // // // //-------strings----//


// // // // // // // // // // // // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // // // // // // // // // // // //     let greetings: String = String::from("Hola");
// // // // // // // // // // // // // // // // // // // // // // // // // // // //     println!("{}", greetings);                      //basic string
// // // // // // // // // // // // // // // // // // // // // // // // // // // //     print!("{}", greetings.chars().nth(1000))        //actual fun part  this is not possble is rust...try pattern matching(some if char or none if overflow) later

// // // // // // // // // // // // // // // // // // // // // // // // // // // // }



// // // // // // // // // // // // // // // // // // // // // // // // // // // //-----loops------


// // // // // // // // // // // // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // // // // // // // // // // // //     for i in 0..10 {
// // // // // // // // // // // // // // // // // // // // // // // // // // // //         println!("{}", i);
// // // // // // // // // // // // // // // // // // // // // // // // // // // //     }

// // // // // // // // // // // // // // // // // // // // // // // // // // // // }



// // // // // // // // // // // // // // // // // // // // // // // // // // // //......iteration....

// // // // // // // // // // // // // // // // // // // // // // // // // // // fn get_first_word(sentence: String) -> String {
// // // // // // // // // // // // // // // // // // // // // // // // // // //     let mut ans: String = String::from("");
// // // // // // // // // // // // // // // // // // // // // // // // // // //     for char in sentence.chars() {
// // // // // // // // // // // // // // // // // // // // // // // // // // //         if char == ' ' {
// // // // // // // // // // // // // // // // // // // // // // // // // // //             break;
// // // // // // // // // // // // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // // // // // // // // // // // //         ans.push_str(char.to_string().as_str());
// // // // // // // // // // // // // // // // // // // // // // // // // // //     }

// // // // // // // // // // // // // // // // // // // // // // // // // // //     return ans;
// // // // // // // // // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // // // // // // // // // // // //     let sentence = String::from("Hello world!");
// // // // // // // // // // // // // // // // // // // // // // // // // // //     let first_word = get_first_word(sentence);
// // // // // // // // // // // // // // // // // // // // // // // // // // //     println!("first word is: {}", first_word);
// // // // // // // // // // // // // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // // // // // // // // // // // ######------Rust-Jargons------######

// // // // // // // // // // // // // // // // // // // // // // // // // // <------Mutability------>
// // // // // // // // // // // // // // // // // // // // // // // // // //  fn main(){
// // // // // // // // // // // // // // // // // // // // // // // // // //     stack_fn();
// // // // // // // // // // // // // // // // // // // // // // // // // //     heap_fn();
// // // // // // // // // // // // // // // // // // // // // // // // // //     update_str_fn();
// // // // // // // // // // // // // // // // // // // // // // // // // //  }

// // // // // // // // // // // // // // // // // // // // // // // // // //  fn stack_fn(){
// // // // // // // // // // // // // // // // // // // // // // // // // //     let x=1;
// // // // // // // // // // // // // // // // // // // // // // // // // //     let y=68;
// // // // // // // // // // // // // // // // // // // // // // // // // //     let z=x+y;
// // // // // // // // // // // // // // // // // // // // // // // // // //     println!("stack function:the sum of {} and {} is {}",x,y,z);
// // // // // // // // // // // // // // // // // // // // // // // // // //  }

// // // // // // // // // // // // // // // // // // // // // // // // // //  fn heap_fn(){
// // // // // // // // // // // // // // // // // // // // // // // // // //     let f1=String::from ("hello");
// // // // // // // // // // // // // // // // // // // // // // // // // //     let f2=String::from ("rust");
// // // // // // // // // // // // // // // // // // // // // // // // // //     let f=format!("{} {}", f1, f2);
// // // // // // // // // // // // // // // // // // // // // // // // // //     println!("Heap function: Combined string is '{}'", f);
// // // // // // // // // // // // // // // // // // // // // // // // // //  }


// // // // // // // // // // // // // // // // // // // // // // // // // //  fn update_str_fn(){
// // // // // // // // // // // // // // // // // // // // // // // // // //     let mut f = String:: from ("initial");
// // // // // // // // // // // // // // // // // // // // // // // // // //     println!("capacity:{}, Length:{}, pointer:{:p}", f.capacity(), f.len(), f.as_ptr());

// // // // // // // // // // // // // // // // // // // // // // // // // // for _ in 0..100{
// // // // // // // // // // // // // // // // // // // // // // // // // //     f.push_str(" and some additional text");
// // // // // // // // // // // // // // // // // // // // // // // // // //     println!("capacity:{}, Length:{}, pointer:{:p}", f.capacity(), f.len(), f.as_ptr()); // delibratly tried to change the pointer->
// // // // // // // // // // // // // // // // // // // // // // // // // // }  
// // // // // // // // // // // // // // // // // // // // // // // // // // }



// // // // // // // // // // // // // // // // // // // // // // // // // // <------Ownership (In a nushell automatic deallocation)------>
// // // // // // // // // // // // // // // // // // // // // // // // //   fn main(){
// // // // // // // // // // // // // // // // // // // // // // // // //     let x1= String::from ("why rust!");
// // // // // // // // // // // // // // // // // // // // // // // // //     let x2=x1;     //extremely memory safe...

// // // // // // // // // // // // // // // // // // // // // // // // //     println!("{}", x2)    // error x1 is no longer a bf. new owner is x2.
// // // // // // // // // // // // // // // // // // // // // // // // //   }


// // // // // // // // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // // // // // // // //   let mut my_string=String::from("hey");
// // // // // // // // // // // // // // // // // // // // // // // //   // takes_ownership(my_string.clone());                //you can use clone to fix below error but its kind a expenisve operation and its not feasible everytime.
// // // // // // // // // // // // // // // // // // // // // // // //   my_string=takes_ownership(my_string);    //takes ownership
// // // // // // // // // // // // // // // // // // // // // // // //   println!("{}", my_string);                           //<--ERROR: my_string is moved 
// // // // // // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // // // // // fn takes_ownership(some_stirng:String) -> String{
// // // // // // // // // // // // // // // // // // // // // // // //   println!("{}", some_stirng);
// // // // // // // // // // // // // // // // // // // // // // // //   return some_stirng;                 //----> And she is back don't forget 3 step process..

// // // // // // // // // // // // // // // // // // // // // // // ///---> one more thing if she get back 
// // // // // // // // // // // // // // // // // // // // // // // /// let my_string_3=same
// // // // // // // // // // // // // // // // // // // // // // // /// println(same) ---> what did we learn rehana is a whore...
// // // // // // // // // // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // // // // // // // // //----->>Borrowing anf refrences...  You know like casual stuff

// // // // // // // // // // // // // // // // // // // // // // // fn main(){   
// // // // // // // // // // // // // // // // // // // // // // //     let x1= String::from ("hello");
// // // // // // // // // // // // // // // // // // // // // // //     let x2= &x1;
// // // // // // // // // // // // // // // // // // // // // // //     println!("{}", x2);
// // // // // // // // // // // // // // // // // // // // // // //     println!("{}", x1);             // she is just borrowed for while Let's See what future holds for us.
// // // // // // // // // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // // // // // //   let mut x1=String::from("hey");
// // // // // // // // // // // // // // // // // // // // // //   update_str_fn(&mut x1);              //mutable refrence... 
// // // // // // // // // // // // // // // // // // // // // //   println!("{}", x1)
// // // // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // // // fn update_str_fn(x:&mut String){            //takes mutable string
// // // // // // // // // // // // // // // // // // // // // //   x.push_str("Product Manager")
// // // // // // // // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // // // // // // //------->>>Let's Make borrower memory safe...

// // // // // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // // // // //      let mut x1=String::from("hey");
// // // // // // // // // // // // // // // // // // // // //      let x2: &mut String =&mut x1;
// // // // // // // // // // // // // // // // // // // // //      x2.push_str("rust");              //only one mut at a time only x2. Can push no x3 &x1 or &mut x1
// // // // // // // // // // // // // // // // // // // // //      println!("{}", x1)                       //another way to pass mutable refrence...
// // // // // // // // // // // // // // // // // // // // //   }



// // // // // // // // // // // // // // // // // // // // ///Now the  easy peasy part
// // // // // // // // // // // // // // // // // // // // /// 1- Struct (ex-objects in JS)
// // // // // // // // // // // // // // // // // // // // struct User{
// // // // // // // // // // // // // // // // // // // //   name:String,
// // // // // // // // // // // // // // // // // // // //   age:u32,
// // // // // // // // // // // // // // // // // // // //   active:bool
// // // // // // // // // // // // // // // // // // // // }
// // // // // // // // // // // // // // // // // // // // fn main(){                // --> Rust book there are also tuple struct as well as unit struct.
// // // // // // // // // // // // // // // // // // // //   let name = String::from("Bobby");
// // // // // // // // // // // // // // // // // // // //   let user= User{
// // // // // // // // // // // // // // // // // // // //     name:name,
// // // // // // // // // // // // // // // // // // // //     age:69,
// // // // // // // // // // // // // // // // // // // //     active:true

// // // // // // // // // // // // // // // // // // // //   };
// // // // // // // // // // // // // // // // // // // //   println!("{} is {} years old", user.name, user.age)
// // // // // // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // // // // ///------> let's have enum's just like typescript.
// // // // // // // // // // // // // // // // // // // //when you have various variants of something...
// // // // // // // // // // // // // // // // // // // // enum Direction{
// // // // // // // // // // // // // // // // // // // //   North,
// // // // // // // // // // // // // // // // // // // //   South, 
// // // // // // // // // // // // // // // // // // // //   East, 
// // // // // // // // // // // // // // // // // // // //   West,
// // // // // // // // // // // // // // // // // // // // }
// // // // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // // // //   let my_direction=Direction::South;
// // // // // // // // // // // // // // // // // // // //   let new_direction = my_direction;
// // // // // // // // // // // // // // // // // // // //   move_around(new_direction);

// // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // // fn move_around(direction:Direction){

// // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // /// one more examples of ENUM
// // // // // // // // // // // // // // // // // // // /// 
// // // // // // // // // // // // // // // // // // // enum DC{
// // // // // // // // // // // // // // // // // // //   Merc,
// // // // // // // // // // // // // // // // // // //   Beamer,
// // // // // // // // // // // // // // // // // // //   Autobio,
// // // // // // // // // // // // // // // // // // //   A8,
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // // //   let my_dc=DC::Merc;
// // // // // // // // // // // // // // // // // // //   let fav_dc=my_dc;
// // // // // // // // // // // // // // // // // // //   test_opt(fav_dc);
// // // // // // // // // // // // // // // // // // //   // println!("{}", fav_dc) for printing this you need add #[derive(Debug)] // Add Debug to print the enum than use the borrow ownership stuff..
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // fn test_opt(dc:DC){
// // // // // // // // // // // // // // // // // // //   //whatever
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // pattern matching---> Let you pattern match across various variants of an enum andgrun some logic


// // // // // // // // // // // // // // // // // //  enum Shape{
// // // // // // // // // // // // // // // // // //   Circle(f64),
// // // // // // // // // // // // // // // // // //   Square(f64),
// // // // // // // // // // // // // // // // // //   Rectangle(f64, f64),
// // // // // // // // // // // // // // // // // //  }

// // // // // // // // // // // // // // // // // //  fn calculate_area(shape:Shape)->f64{

// // // // // // // // // // // // // // // // // //   let ans= match shape{
// // // // // // // // // // // // // // // // // //     Shape::Circle(radius) => 3.14*radius*radius,
// // // // // // // // // // // // // // // // // //     Shape::Rectangle(height, width)=> height*width,
// // // // // // // // // // // // // // // // // //     Shape::Square(side)=> side*side
// // // // // // // // // // // // // // // // // //   };
// // // // // // // // // // // // // // // // // //   return ans;
// // // // // // // // // // // // // // // // // //  }


// // // // // // // // // // // // // // // // // //  fn main(){
// // // // // // // // // // // // // // // // // //   let circle=Shape::Circle(5.0);
// // // // // // // // // // // // // // // // // //   let square=Shape::Square(4.0);
// // // // // // // // // // // // // // // // // //   let rectangle=Shape::Rectangle(3.0, 6.0);

// // // // // // // // // // // // // // // // // // let area= calculate_area(square);
// // // // // // // // // // // // // // // // // // print!("Area of square:{}\n", area)
// // // // // // // // // // // // // // // // // //  }


// // // // // // // // // // // // // // // // // //---> error handling...

// // // // // // // // // // // // // // // // // //genrics is capital letters.. 

// // // // // // // // // // // // // // // // // //result enum 

// // // // // // // // // // // // // // // // //   //X can be anything integer, string, etc.. no restrictions.
// // // // // // // // // // // // // // // // //   //Y also can be anything but Y as a struct usually follows certain format.

// // // // // // // // // // // // // // // // // // Let's take an example of reading file..



// // // // // // // // // // // // // // // // // use std::fs;

// // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // //     // Read the file content using the custom function
// // // // // // // // // // // // // // // // //     let res = read_from_file_unsafe("examples.txt".to_string());
// // // // // // // // // // // // // // // // //     print!("hey");

// // // // // // // // // // // // // // // // //     // Match the result
// // // // // // // // // // // // // // // // //     match res {
// // // // // // // // // // // // // // // // //         Ok(content) => {
// // // // // // // // // // // // // // // // //             println!("file content: {}", content);
// // // // // // // // // // // // // // // // //         },
// // // // // // // // // // // // // // // // //         Err(err) => {
// // // // // // // // // // // // // // // // //             println!("error: {}", err);
// // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // fn read_from_file_unsafe(file_name: String) -> Result<String, String> {
// // // // // // // // // // // // // // // // //     // Attempt to read the file content
// // // // // // // // // // // // // // // // //     let res = fs::read_to_string(&file_name); // Use the passed file name

// // // // // // // // // // // // // // // // //     // Handle the result
// // // // // // // // // // // // // // // // //     if let Ok(content) = res {
// // // // // // // // // // // // // // // // //         Ok(content) // Return the content if successful
// // // // // // // // // // // // // // // // //     } else {
// // // // // // // // // // // // // // // // //         Err("error loading file".to_string()) // Return a custom error message if failed
// // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // }






// // // // // // // // // // // // // // // // //---->option Enum 

// // // // // // // // // // // // // // // // // pub enum Option<T> {
// // // // // // // // // // // // // // // // //   None,
// // // // // // // // // // // // // // // // //   Some(T),
// // // // // // // // // // // // // // // // // }



// // // // // // // // // // // // // // // // fn find_first_a(s: String) -> Option<i32> {
// // // // // // // // // // // // // // // //   for (index, character) in s.chars().enumerate() {
// // // // // // // // // // // // // // // //       if character == 'a' {
// // // // // // // // // // // // // // // //           return Some(index as i32);  //since return type is Optioni32 we have to return the Some(index) or ok(index) at some places.  
// // // // // // // // // // // // // // // //       }
// // // // // // // // // // // // // // // //   }
// // // // // // // // // // // // // // // //   return None;
// // // // // // // // // // // // // // // // }  


// // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // //   let my_string = String::from("raman");
// // // // // // // // // // // // // // // //   let res=find_first_a(my_string);
// // // // // // // // // // // // // // // //   match res{
// // // // // // // // // // // // // // // //       Some(index) => println!("The letter 'a' is found at index: {}", index),
// // // // // // // // // // // // // // // //       None => println!("The letter 'a' is not found in the string."),
// // // // // // // // // // // // // // // //   }
// // // // // // // // // // // // // // // // }



// // // // // // // // // // // // // // // //---------->This is half of Basic Rust------------</


// // // // // // // // // // // // // // // //cargo libs and some other misc jargons..


// // // // // // // // // // // // // // // // use rand::{rng, Rng};

// // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // //     let mut rng=rng();
// // // // // // // // // // // // // // // //     let n: u32 = rng.random();
// // // // // // // // // // // // // // // //     println!("Random number: {}", n);
// // // // // // // // // // // // // // // // }  generate random numbers with updated syntax as per 2k24. thread_gen() is deprecated 





// // // // // // // // // // // // // // // // Date & Time stuff.... Chronon is the way.
// // // // // // // // // // // // // // // // use chrono::{Local, Utc};

// // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // //     // Get the current date and time in UTC
// // // // // // // // // // // // // // // //     let now = Utc::now();
// // // // // // // // // // // // // // // //     println!("Current date and time in UTC: {}", now);

// // // // // // // // // // // // // // // //     // Format the date and time
// // // // // // // // // // // // // // // //     let formatted = now.format("%Y-%m-%d %H:%M:%S");
// // // // // // // // // // // // // // // //     println!("Formatted date and time: {}", formatted);

// // // // // // // // // // // // // // // //     // Get local time
// // // // // // // // // // // // // // // //     let local = Local::now();
// // // // // // // // // // // // // // // //     println!("Current date and time in local: {}", local);
// // // // // // // // // // // // // // // // }





// // // // // // // // // // // // // // // // What all libraries does rust have?
// // // // // // // // // // // // // // // // A lot of them
// // // // // // // // // // // // // // // // 1. https://actix.rs/ - Extremely fast http server.
// // // // // // // // // // // // // // // // 2. https://serde.rs/ - Serializing and deserialising data in rust.
// // // // // // // // // // // // // // // // 3. https://tokio.rs/ - Asynchronous runtime in rust.
// // // // // // // // // // // // // // // // 4. https://docs.rs/reqwest/latest/reqwest/ - Send HTTP requests.
// // // // // // // // // // // // // // // // 5. https://docs.rs/sqlx/latest/sqlx/ - Cornect to sql database.




// // // // // // // // // // // // // // // // Leftovers - Traits, Generics and Lifetimes, Multithreading, macros, async ops (Futures)
// // // // // // // // // // // // // // // // https://doc.rust-lang.org/book/ch10-00-generics.html.
// // // // // // // // // // // // // // // // https://doc.rust-lang.org/book/ch19-00-advanced-features.html.
// // // // // // // // // // // // // // // // https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html.
// // // // // // // // // // // // // // // // https://doc.rust-lang.org/std/future/trait.Future.html.


// // // // // // // // // // // // // // // // what can you build with this 
// // // // // // // // // // // // // // // //Right Now: CLI's, Backend for your fullstack application.(with some async knowledge). 


// // // // // // // // // // // // // // // // some Basic question...

// // // // // // // // // // // // // // // //-------> Even Number
// // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // //   let ans = is_even(69);
// // // // // // // // // // // // // // // //   println!("{}", ans);
// // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // fn is_even(num:i32) ->bool{
// // // // // // // // // // // // // // // //   if num%2==0{
// // // // // // // // // // // // // // // //     true;
// // // // // // // // // // // // // // // //   }
// // // // // // // // // // // // // // // //   return false;

// // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // -------> Fibbonacci Shit--<
// // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // //   let ans=fib(5);
// // // // // // // // // // // // // // // //   println!("{}", ans);
// // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // fn fib(num: u32)->u32{
// // // // // // // // // // // // // // // //   let mut first= 0;
// // // // // // // // // // // // // // // //   let mut second= 1;

// // // // // // // // // // // // // // // //   if num == 0{
// // // // // // // // // // // // // // // //       return first;
// // // // // // // // // // // // // // // //   }
// // // // // // // // // // // // // // // //   if num == 1{
// // // // // // // // // // // // // // // //       return second;
// // // // // // // // // // // // // // // //   }

// // // // // // // // // // // // // // // //   for _ in 0..(num - 1) { // Fixed loop range
// // // // // // // // // // // // // // // //       let temp = second;
// // // // // // // // // // // // // // // //       second = second + first;
// // // // // // // // // // // // // // // //       first = temp;
// // // // // // // // // // // // // // // //   }
// // // // // // // // // // // // // // // //   return second;
// // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // fn get_string_by_length(s: &str)->usize{
// // // // // // // // // // // // // // // //     s.chars().count()    //implicit return...w/o ;
// // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // //     let my_string=String::from("bye Rust");
// // // // // // // // // // // // // // // //     let length = get_string_by_length(&my_string);
// // // // // // // // // // // // // // // //     println!("Number of char: {}", length)
// // // // // // // // // // // // // // // // }  


// // // // // // // // // // // // // // // //-----> pattern matchingn using option enum 
// // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // //     let index=find_first_a(String::from("harmanpreet"));
// // // // // // // // // // // // // // // //     match index  {
// // // // // // // // // // // // // // // //         Some(value) => println!(" index is {}", value),
// // // // // // // // // // // // // // // //         None=>println!("a not found"),
// // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // fn find_first_a(s: String)->Option<i32>{

// // // // // // // // // // // // // // // //     for (index, char) in s.chars().enumerate(){
// // // // // // // // // // // // // // // //         if char == 'a'{
// // // // // // // // // // // // // // // //             return Some(index as i32);

// // // // // // // // // // // // // // // //         } 
// // // // // // // // // // // // // // // //      }
// // // // // // // // // // // // // // // //     return None;

// // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // //####----->Since Developers are dumb so we force them to write the code a certain way to avoid memory leak issues like dangling pointers in C.-------#Raii pattern memory Clenaup..



// // // // // // // // // // // // // // // // Moving a varible-----ownership scope in a nutshell...or borrowing out of scope.


// // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // //     let a1=String::from ("hey");
// // // // // // // // // // // // // // //     let a2=a1;
// // // // // // // // // // // // // // //     // println!("{}", a1);     // if you print a1 here you will end facing borrow issue.. U can go with clone to get it working and waste memory at 2 places for same stuff...
// // // // // // // // // // // // // // //     println!("{}", a2);      
// // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // //        let a1=String::from ("hey");
// // // // // // // // // // // // // // //        let a2=&a1;    // if you stll want to rent your Rehana you do reference a1, a2 is ref the same string. 
// // // // // // // // // // // // // // //        println!("{}", a1);    
// // // // // // // // // // // // // // //        println!("{}", a2);   
// // // // // // // // // // // // // // // }-----> No hanky-panky is still better example....



// // // // // // // // // // // // // // //-------------------------------------------let's Step-up------------------------------------------------------------------------------





// // // // // // // // // // // // // // //---->Collections (dynamic stuff storage on Heap offcourse )..


// // // // // // // // // // // // // // //----1) vectors-- Dynamic array

// // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // //     let mut vec = Vec::new();
// // // // // // // // // // // // // //         vec.push(1);
// // // // // // // // // // // // // //         vec.push(12);
// // // // // // // // // // // // // //         vec.push(13);
// // // // // // // // // // // // // //         vec.push(3);
// // // // // // // // // // // // // //         println!("{:?}", vec);     //implementing debug trait

// // // // // // // // // // // // // //     }



// // // // // // // // // // // // // //ques: Write a function that takes the vectors as the input and returns the vectors with even values.

// // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // //         let mut vec = Vec::new();
// // // // // // // // // // // // //             vec.push(1);
// // // // // // // // // // // // //             vec.push(12);
// // // // // // // // // // // // //             vec.push(13);
// // // // // // // // // // // // //             vec.push(3);

// // // // // // // // // // // // //     print!("{:?}", vec_eat(vec));
// // // // // // // // // // // // // }

// // // // // // // // // // // // // fn vec_eat(vec:Vec<i32>) -> Vec<i32> {
// // // // // // // // // // // // //     let mut new_vec=Vec::new();
// // // // // // // // // // // // //     for val in vec{
// // // // // // // // // // // // //         if val% 2 == 0 {
// // // // // // // // // // // // //         new_vec.push(val);
// // // // // // // // // // // // //         }
// // // // // // // // // // // // //     }
// // // // // // // // // // // // //     return new_vec;
// // // // // // // // // // // // // }


// // // // // // // // // // // // // //----> initializing the vector using rust macros.

// // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // //     let numbers=vec![1,2,3,4,5];
// // // // // // // // // // // // //     for number in numbers{
// // // // // // // // // // // // //         print!("{}", number)
// // // // // // // // // // // // //     }
// // // // // // // // // // // // // }




// // // // // // // // // // // // //-------->Hashmaps(Store key value pairs rs:23Y)-------<
// // // // // // // // // // // // //Inser, Get, Remove, Clear.

// // // // // // // // // // // // use std::collections::HashMap;

// // // // // // // // // // // // fn main(){
// // // // // // // // // // // //     let mut users=HashMap::new();
// // // // // // // // // // // //     users.insert(String::from("Rs"), 23);
// // // // // // // // // // // //     users.insert(String::from("Js"), 23);
    

// // // // // // // // // // // // let first_user_age = users.get("Rs");
// // // // // // // // // // // // match first_user_age {
// // // // // // // // // // // //     Some(age)=>println!("age is {}", age),
// // // // // // // // // // // //     None=>println!("user not found"),
    
// // // // // // // // // // // // }

// // // // // // // // // // // // } 


// // // // // // // // // // // // Write a function that takes a vector of tuples (each tuple containing a key and a value) and returns a Hashmap where the kevs are the unique keys from the input tuples and the values are vectors of all corresponding values associated with each key.



// // // // // // // // // // // use std::collections::HashMap;

// // // // // // // // // // // fn group_value_by_key(vec:Vec<(String, i32)>)->HashMap<String, i32>{
// // // // // // // // // // //     let mut hm=HashMap::new();
// // // // // // // // // // //     for (key, value)in vec{
// // // // // // // // // // //         hm.insert(key,value);
// // // // // // // // // // //     }
// // // // // // // // // // //     return hm;
// // // // // // // // // // // }

// // // // // // // // // // // fn main(){
// // // // // // // // // // //     let input_vec = vec![(String::from("Sharma"),23), (String::from("Bros"), 69)];
// // // // // // // // // // //     let hm = group_value_by_key(input_vec);
// // // // // // // // // // //     println!("{:?}", hm);
// // // // // // // // // // // }


// // // // // // // // // // /*------You could have also try using struct like
// // // // // // // // // // Struct User{
// // // // // // // // // // name: string,
// // // // // // // // // // age:i32, 

// // // // // // // // // // fn whatevr(vec:Vec<User>)
// // // // // // // // // // }----------------------------------*/



// // // // // // // // // // //-----------------------iterators------------------------->


// // // // // // // // // //simple iterator---->
// // // // // // // // // fn main(){
// // // // // // // // //     let nums = vec![1,2,3];

// // // // // // // // //     for val in nums{
// // // // // // // // //         println!("{}", val);
// // // // // // // // //     }
// // // // // // // // // }

// // // // // // // // //After creating an iterator of your own...

// // // // // // // // // // fn main(){
// // // // // // // // // //     let v1 = vec![1,2,3,];
// // // // // // // // // //     // let v1_iter = v1.iter();  // your own iterator, here v1_iter jsut borrowed the v1 values doesn't became the v1...
// // // // // // // // // // for val in v1{
// // // // // // // // // //     print!("got:{val}");
// // // // // // // // // //     }
// // // // // // // // // // }

// // // // // // // // //------> mutable refrence

// // // // // // // // // fn main(){
// // // // // // // // //     let mut v1 = vec![1,2,3];
// // // // // // // // //     let v1_iter = v1.iter_mut();

// // // // // // // // //     for val in v1_iter{
// // // // // // // // //         *val = *val+3;
// // // // // // // // //     }
// // // // // // // // //     print!("{:?}", v1);
// // // // // // // // // }


// // // // // // // // //------using .next------>

// // // // // // // // // fn main(){
// // // // // // // // //     let nums=vec![1,2,3];
// // // // // // // // //     let mut iter =nums.iter();

// // // // // // // // //     while let Some(val) = iter.next(){
// // // // // // // // //         print!("{}", val);
// // // // // // // // //     }
// // // // // // // // // }




// // // // // // // // //------>Into-iter-------------------
// // // // // // // // /*The Intolterator trait is used to convert a collection into an iterator that takes ownership of the collection.
// // // // // // // // Useful when
// // // // // // // // 1. You no longer need the original collection
// // // // // // // // 2. When you need to squeeze performance benefits by transferring ownership (avoiding references) */

// // // // // // // // use core::prelude::v1;

// // // // // // // // fn main () {
// // // // // // // //     let v1 = vec! [11, 2, 31];
// // // // // // // //     let v1_iter = v1.into_iter();
// // // // // // // //     for val in v1_iter {
// // // // // // // //         println! ("{}", val);
// // // // // // // //     }
// // // // // // // //     // println! ("{:?}", v1);      // u can't use v1 anymore... v1 moved to v1_iter
// // // // // // // //     //println! ("{:?}", v1_iter);  // u can't use v1_iter anymore... v1_iter moved to val
// // // // // // // // }


// // // // // // // // /* Which to chose?

// // // // // // // // ----Iter---
// // // // // // // // If you want immutable references to the inner variables and don't want to transfer ownership
// // // // // // // // ---IterMut----
// // // // // // // // If you want mutable references to the inner variables and don't want to transfer ownership
// // // // // // // // ---IterInto---
// // // // // // // // If you want to move the variable into the iterator and don't want to use it afterwards */


// // // // // // // //  //-------Consuming adapters......


// // // // // // //  fn main(){
// // // // // // //     let v1 = vec![1,2,4];
// // // // // // //     let v1_iter=v1.iter();
// // // // // // //     let total:i32=v1_iter.sum();

// // // // // // //     print!("sum  is {}", total);

// // // // // // //     for i in v1_iter{}   // v1_iter can't be used its been eaten up..

// // // // // // //     // aseert_eq!(total, 6);
// // // // // // //     // let sum2=v1_iter;

// // // // // // //  }


// // // // // // //-----> iterator adaptor----->similar to map in JS



// // // // // // fn main(){
// // // // // //     let v1=vec![1,2,3];                 // here is the OG vector 
// // // // // //     let v1_iter = v1.iter();      //  here is hte iter 1
// // // // // //     let v1_iter2=v1_iter.map(|x| x+1);      //  here is hte iter2 which maps over iter1 and inc the value of og vec by 1 and store it in iter2

// // // // // //     for i in v1_iter2{
// // // // // //         println!("{}", i)
// // // // // //     }

// // // // // //     println!("original vector{:?}", v1);    // still intact. 
// // // // // // }


// // // // // //-------->filter----->

// // // // // //filterOut Even 
// // // // // fn main(){

// // // // //     let v1=vec![1,2,4];
// // // // //     let v1_iter=v1.iter();
// // // // //     let v1_iter2=v1_iter.filter(|x| *x%2==0);   // ref  ideal **x
// // // // //       for i in v1_iter2{
// // // // //         println!("new values {:?}", i);
// // // // //       }

// // // // //--------> Strings vs Slices....



// // // // // Write a function that takes a string as an input And returns the first word from it


// // // // //slices----->
// // // // // fn main(){
// // // // //     let word =String::from("Hey Rust");
// // // // //     let word2=&word[0..4];

// // // // //     println!("{}", word2)
// // // // // }

// // // // fn main(){
// // // //     let mut word =String::from("Hey Rust");
// // // //     let word2=find_first_word(&word);    
// // // //     println!("{}", word);
// // // //     println!("{}", word2);
// // // // }


// // // // fn find_first_word(word:&String)->&str{       //returning string slice
// // // //     let mut index=0;
// // // //     for (_, i) in word.chars().enumerate(){
// // // //         if i == ' ' {
// // // //             break;
// // // //         }
// // // //         index=index+1;
// // // //     }
// // // //     return &word[0..index];
// // // // }
// // // // // you might think its fine can't ref word  dangling pointer 

// // //-------> Approach 2 with slices....
// // // fn main() {
// // //     let name = String::from("hello world");
// // //     // ans = &name [0.. index], ans is an immutable reference to name
// // //     let mut space_index = 0;

// // //     for i in name.chars() {
// // //         if i == ' ' { 
// // //             break;
// // //         }
// // //         space_index = space_index + 1; 
// // //     }

// // //     let ans = &name[0..space_index]; 
// // //     println!("ans is {}", ans); 
// // // }



// // fn main () {
// //     let name = String:: from ("hello world"); // String type
// //     let string_slice = &name; // Has a view' into the original string/is a reference
// //     let string_literal = "hello"; // literal is also an &str but it points directly to an address
// // //in the binary
// // }


// ///-----> generics...
// fn main(){
//     let bigger =largest(1,2);
//     let bigger_char=largest('a', 'b');
//     println!("{}", bigger);
//     println!("{}", bigger_char);
// }

// fn largest<T: std::cmp::PartialOrd>(a:T, b:T)->T{
//     if a>b{
//         a
//     }
//     else {
//         b
//     }
// }


//-------> Traits/ Abs Class in Java / Intefaces in JS. 


//---->create Trait
trait  Summary{
    fn summarize(&self)->String;
    
}
///Creat Struct
struct User{
    name: String,
    age: u32,
}
//Implement Struct

impl Summary for User {
    fn summarize(&self)->String {
        return format!("the name is {}, and age is {}", self.name, self.age);
    }
    
}

fn main(){
    let user = User{
        name:String::from("Rusitng"),
        age:90
    };
    print!("{}", user.summarize());
}






