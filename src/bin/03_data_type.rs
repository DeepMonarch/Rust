fn main() {
    // 1) Scalar Types: Integer, Float, Bool, Character

    // Float Type
    let float32_number: f32 = 3.14; //f32 floating point number 
    let float64_number = 6.32; // automatically assumes f64

    println!("float data types: {}, {}", float32_number, float64_number);

    // Bool Type
    let is_raining:bool = true; 
    let is_sunny = false; 

    let need_umbrella = is_raining && !is_sunny;
    let need_glasses = is_raining || is_sunny;

    println!("bool data type: {}, {}, {}, {}", is_raining, is_sunny,need_umbrella, need_glasses);

    // Character 4 bytes (32 bits)
    let ascii: char = 'A';
    let emoji: char = '🔥';
    let kanji: char = '日';

    println!("ASCII : {}", ascii);
    println!("Emoji : {}", emoji);
    println!("Kanji : {}", kanji);

    // 2) Compound Types: Primitive Types, Complex Types
    //Primitive Type: array, tuple
    let mut array:[u8;5];
    array=[1,2,3,4,5];

    println!("array[0]={}", array[0]);
    array[2] = 30;
    println!("array[2]={}", array[2]);

    println!("Array length is {}", array.len());

    // can be directly passed in a function or a reference of it can be passed 
    write_array(&mut array);
    println!("array :{:#?}", array); // for column view use {:#?} for row view use {:?}

    //Complex: string, vector

    // let mut v:Vec<i32> = Vec::new(); //declaration 
    // another way
    let mut v = Vec::<i32>::new();
    
    v.push(1);
    v.push(2);
    v.push(3);

    // start with some initial values
    let mut vector = vec![1,2,3,4,5];
    
    vector.pop(); // deleting last value

    println!("v ={:?}", v);
    println!("Vector ={:?}", vector); 

    // while using vectors we can not directly give a vector as parameter because ownership is applied as it is heap based memory 

    // write_vector(vector); // would give an error
    // use write_vector(vector.clone()); // or a mutable reference 

    shadowing();

    // if -else

    let number = 12;
    if number % 3 == 0 || number % 4 == 0 {
        println!("The number is divisible by either 3 and 4");
    }
    else {
        println!("Isn't divisible by either");
    }

}

// option 1 - passing array itself as a parameter

// fn write_array(mut arr:[u8;5]) { // arr is new copy of array
//     arr[0] = 9;
//     println!("arr={:?}", arr);
// }

// option 2 - passing reference of array as a parameter

fn write_array(array:&mut [u8;5]){
      array[0] = 9;
      println!("arr={:?}", array);
}

// Better less expensive option is to use reference instead of directly using the array that creates a copy of the array
// whether it is read or write operation

// Shadowing 

fn shadowing(){
    let x = 5;
    println!("{}",x);
    let x = "Hellooo";
    println!("{}",x);
    let x = x.len(); 
    println!("{}",x);
}