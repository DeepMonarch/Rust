fn main() {
    // loop : an infinite loop runs until you provide a break condition
    loop {
        println!("Hello loop");
        break;
    }
    // while loop 
    let mut count = 0;

    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
    // for loop
    let arr=[1,2,3];

    for i in &arr {
        println!("{}", i);
    }

    // match
    let number = 6;

    match number {
        x if is_even(x)=>println!("Even"),

        1=>println!("Number is one"),
        2 | 4=>println!("Number is two of four"), // 2 or 4
        5=>println!("Number is five"),
        _=>println!("Number is not recognizable") // default case // necessary

    }
}

fn is_even(num:i8)->bool{
    return num%2==0;
    // if num%2 == 0 {
    //     return true;
    // }
    // else {
    //     return false;
    // }
}