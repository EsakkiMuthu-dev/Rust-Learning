use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("<--- Lets start the game --- >");

    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess =  String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to red");
        let guess:u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };

        match guess.cmp(&random_number){
            Ordering::Greater => println!("Too great "),
            Ordering::Less => println!("Too Less"),
            Ordering::Equal =>{
                println!("You win ! ");
                break;
            }
        }
    }

}
