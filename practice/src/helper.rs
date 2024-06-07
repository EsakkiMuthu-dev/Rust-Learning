use std::fmt::format;
use std::io::stdin;
// helper function to get user input
// helper::get_user_input(prompt : &str)


pub fn get_user_input(prompt : &str) -> String{
    println!("{prompt}");
    let mut input = String:: new();
    stdin().read_line(&mut input).expect("Unable to get user input");
    input
}

pub fn get_numbers(numbers_count : i32) -> Option<Vec<i32>>{
    let mut numbers:Vec<i32> = Vec::new();
    for i in 1..=numbers_count{
       match get_number(format!("Enter the number {} : ",i).as_str()){
            Some(n) => numbers.push(n),
            None => return None
        };
    }
    Some(numbers)
}

pub fn get_number(prompt:&str) -> Option<i32> {
    let number = get_user_input(prompt).trim().parse::<i32>();
    match number {
        Ok(n) => Some(n),
        Err(e) => None
    }
}