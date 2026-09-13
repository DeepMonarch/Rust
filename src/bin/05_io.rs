use std::io;
fn main() {
    let mut input = String::new();
    println!("User Input Your Name: ");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Input Failed");

    println!("User Input: {}", input);
}