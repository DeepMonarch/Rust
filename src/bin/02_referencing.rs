fn main() {
    let mut x = 5;
    let y = &x; // y is the reference to the value of x, value of x is 5
    // println!("address of x = {:p}", y); //stored address

    println!("{}", y); // auto dereferencing // y ki value na output karte hue y jisko refer kar rha hai usko output karti hai
    println!("{}", *y); // behind the scene using a * operator 

    x = x+1; //6
    let w:&mut i32 = &mut x;
    *w=*w+1; //7 // cannot work without * // address + integer hojayega without *w 
    println!("The value of x: {}",*w); // can work automatically without * operator // auto dereferencing can happen

    /* Dangling Reference */
    // let reference_to_nothing = create_string_ref();
}

// Notes
// A reference holds metadata abuot the reference, such as its lifetime and mutability
// Meanwhile a raw pointer holds the direct memory address
// references are generally safer as ownership rules are applied here

/* Dangling Reference */

// fn create_string_ref()->&String {
//     let s:String = String::from("hello");
//     return &s;
// }