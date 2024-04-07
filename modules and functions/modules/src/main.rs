pub mod helper;
pub mod closures;

fn main() {
    println!("Hello, world!");
    let word = helper::sorry_word::join_wrds("Sorry", "Bharathi");
    println!("{word}");
    helper::to_it_repeat::ask_sorry_with_sorry_wrd(&word);
    closures::some_fun();
}


