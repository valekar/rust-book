use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is {} ", secret_number);

    loop {
        println!("Please enter your input");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("msds");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
        println!("you guess {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big "),
            Ordering::Less => println!("Too less"),
        }
    }
}
