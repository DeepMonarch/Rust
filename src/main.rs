mod function; // the function.rs file (module)
mod ownership;
fn main() {
    // 1.1 signed and 1.2 unsigned integer
    let mut num:u8 = 5; //unsigned integer 8 bits // 0 to 255 (0 2^n-1) // immutable variable (use mut)
    println!("This is stored in num {}", num); //num ki value {} me aajayegi (format specifiers in c) 
    num=199;
    println!("This is stored in num {}", num); //num ki value {} me aajayegi (format specifiers in c) 
    
    //2.1 &str (String Literal) //fixed length strings // rodata(read only data) me store hoti hai
    let sentence: &str = "Hi, U0ser!";
    println!("Here, {}\n", sentence); //num ki value {} me aajayegi (format specifiers in c) 

    //2.2 String // heap allocated
    let mut example: String = String::from("Hi, coders"); // dynamic length strings // size is changeable
    example.push_str("What's up?"); 
    println!("Here, {}\n", example);

    //3.1 Tuple // multiple types are allowed
    let employee_info:(&str,u8) = ("Ramesh", 50);
    // accessing the values
    let employee_name = employee_info.0;
    let employee_age:u8 = employee_info.1;
    print!("Employee Name={}, Employee Age={}\n", employee_name, employee_age);
    // destructuring // if you dont want to write manually 0,1 here
    let (emp_name, emp_age) = employee_info;
    print!("Employee Name={}, Employee Age={}", emp_name, emp_age);

    //importing the function.rs print_val
    function::main();
    ownership::main();
}

// snake_case: use this for folder/files 
// cargo build // -- compilation (the file is converted into machine code)
//  cargo run  --release  // for production

// type inference (bina :u8 ke bhi smjh jayega, par complex data type pe error dega waha batana zaroori hai)

// Signed Integer - both negative and positive
/*
i8
i16
i32
i64
isize
*/

//Unsigned Integer - only positive value
/*
u8
u16
u32
u64
usize
*/

