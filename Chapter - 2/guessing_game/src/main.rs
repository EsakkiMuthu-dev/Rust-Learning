use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!(" < ----- Welcome to the Guessing game ----- > ");
    
    println!(" please input your guess : ");
    let random_number:i32 = rand::thread_rng().gen_range(1..=100);
    println!(" Random number generated is {random_number}");

    let mut guess = String::new();
    println!(" : {guess} ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readlinr");
    let guess:u32 = 
    match  guess.cmp(&random_number) {
        Ordering::Equal => print!("You win!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("too Great!"),
    }
    println!("you guessed this number :{guess} ");
}