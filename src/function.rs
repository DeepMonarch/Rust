// Functions
pub fn main(){
    print_value(5);

    let num1:u8 = 5;
    let num2:u8 = 9;

    let result:u8 = add(num1,num2);
    println!("Result is {}", result);
}

pub fn print_value(int:u8){
    println!("The int is {}", int);
}

pub fn add(int1:u8, int2:u8)->u8{
return int1+int2;
}