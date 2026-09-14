use std::io;
use rand::RngExt;

fn main() {
    let guess_list = ["grapes", "bananas", "oranges"];
    let mut rng = rand::rng();

    let index = rng.random_range(0..guess_list.len());
    let random_fruit = guess_list[index];

    println!("random_fruit: {}", random_fruit);

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected = input.trim().to_lowercase();

                println!("fruit_selected: {}", fruit_selected);

                if !guess_list.contains(&fruit_selected.as_str()) {
                    println!("Fruit entered does not exist");
                    continue;
                }

                if guess_checker(&fruit_selected, random_fruit) {
                    println!("You are winner");
                    break;
                } else {
                    println!("Retry");
                }
            }

            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

fn guess_checker(fruit_selected: &str, random_fruit: &str) -> bool {
    fruit_selected == random_fruit
}