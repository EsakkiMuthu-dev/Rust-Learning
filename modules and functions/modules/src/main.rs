use std::io;
pub mod helper;
pub mod closures;
pub mod matchtest;
pub mod  Optiontest;
pub mod  mystructc;
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
    let result_string = match Optiontest::give_option() {
        Some(str) => str,
        None => "".to_string()
    };
    println!("{result_string}");
    let result_int = Optiontest::give_option_int().is_none();
    println!("{result_int}");

    let show_name = match Optiontest::give_option_show() {
        Some(show) =>show,
        None => Optiontest::show::GameOfThornes
    }; 
    println!("Favourite show is {:?}",show_name);
    let person = mystructc::create_person();
    println!("first_name {} last_name {} ",person.first_name , person.last_name);
}

