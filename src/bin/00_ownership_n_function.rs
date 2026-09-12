/* Runs without any error */
//STACK
// fn main() {
//     let x:u8 = 5; // x memory
//     process_integer(x);
//     println!("The value of x in main() is {}", x);
// }

// fn process_integer(item:u8){ // item memory
//     println!("The value of x in main() is {}", item);
// } 

// cargo run --bin ownership_n_function

// //HEAP
// fn main() {
//     let x:String = String::from("Hello"); // owner is x of Hello
//     process_string(x); // transfer of ownership 
//     // println!("The value of x in main() is {}", x); // this causes error
// }
// // item = x
// fn process_string(item:String) { // New owner of Hello
//     println!("The value of x in process_string() is {}", item); // runs perfectly in the block
// }



//HEAP  --- SOLUTION 1
// fn main() {
//     let x:String = String::from("Hello"); // owner is x of Hello
//     let (x, s) = process_string(x); // transfer of ownership 
//     println!("{}", s); // new owner created x
//     println!("The value of x in main() is {}", x); 
// }
// // item = x
// fn process_string(item:String) ->(String,String) { // New owner of Hello
//     let s:String = format!("The value of x in process_string is {item}");
//     return (item,s);
// }


//HEAP -- SOLUTION 2
fn main() {
    let x:String = String::from("Hello"); // owner is x of Hello
    process_string(x.clone()); // The clone mehod is use for the deep copy of the heap data, This is an expensive method.
    println!("The value of x in main() is {}", x); // 
}
// item = x
fn process_string(item:String) { // New owner of Hello
    println!("The value of x in process_string() is {}", item); // runs perfectly in the block
}