use std::io;
pub mod helper;
pub mod closures;
pub mod matchtest;

fn main() {
    println!("Hello, world!");
    let word = helper::sorry_word::join_wrds("Sorry", "Bharathi");
    println!("{word}");
    helper::to_it_repeat::ask_sorry_with_sorry_wrd(&word);
    closures::some_fun();
    println!(" Name ur favourite show!! ");
    let mut show_name= String::new();
    io::stdin()
        .read_line(& mut show_name)
        .expect(" Failed to read data");

    matchtest::it_is_favourite_for_you(&show_name.trim().to_lowercase());
}


