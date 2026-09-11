const GLOBAL_CONST:u8 = 7; // Type inference work nahi karta const me // Capital me hi hona chahiye
pub fn main() {
    // Ownership ke through memory management karte hai
    // Allocating, Deallocating, Freeing up the memeory, Control First approach problem(Programmer karta hai memory management):- Dangling Pointers
    // Safety First approach solved using Garbage collector (still caused problems)!
    // Rust solved it usng Ownership
    /* MEMORY MANAGEMENT */
    // 1. Memory Allocation <- (ptr)
    // 2. Later Initialize Memory <- (ptr)
    // 3. Free/Deallocate Memory <- (ptr) -> dangling ptr (contains garbage value) //solved by assigned NULL to the ptr
    // ownership concept is based absolutely on heap

    // Rule 1. each value in rust has a variable that's called its owner
    // Rule 2. There can be only one owner at a time
    // Rule 3. When the owner goes out of the scope the value will be dropped.

    // Scope : ex: ek code block ke andar hi scope hota hai {}

    let outside_variable:u8 = 5;

    {
        let inside_variable:u8 = 6;
        println!("Inside Variable {}", inside_variable);
    }

    println!("Outside Variable {}", outside_variable);
    print_vall();
}

pub fn print_vall(){
    println!("Const value is {}", GLOBAL_CONST);
}

/* ERROR (Ownership Example) // with u8 or simply int it will work */
// fn error(){
//     let str1 = String::from("Hello"); 
//     let str2 = str1; // ownership moved to str2 so str1 is no longer usable 
//     // println!("str 1 = {}", str1); // this makes an error 
//     println!("str 2 = {}", str2); 
// }


/* OWNERSHIP & FUNCTIONs */

