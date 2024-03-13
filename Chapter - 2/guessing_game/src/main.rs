use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!(" < ----- Welcome to the Guessing game ----- > ");
    let random_number:u32 = rand::thread_rng().gen_range(1..=100);
    
    loop
    {
        println!(" please input your guess : ");
        let mut guess = String::new();
        println!(" : {guess} ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readlinr");

       
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {guess} ");
        match guess.cmp(&random_number) {
            Ordering::Equal =>{ 
            print!("You win!");
            break;
            },
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("too Great!"),
        };
    }
}