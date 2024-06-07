use std::fmt::Error;
use std::io::ErrorKind;
use std::io::ErrorKind::InvalidData;
use log::error;
use crate::helper;
use crate::helper::get_user_input;

// use crate
pub fn run(){
    println!("<------- Welcome to Simple Calculator ------------ >");
    loop {
        println!();
        println!(" Happy to help you using rust!🦀");
        println!("Options: ");
        println!("1.Addition");
        println!("2.Subtraction");
        println!("3.Multiplication");
        println!("4.Division");
        println!();

        let mut option = get_user_input("Choose the Option to perform").trim().parse::<u8>();

        match option{
            Ok(1..=4) => match  handle_operation(option.unwrap())  {
                Ok(n) => println!("<--------------->"),
                Err(e) => { println!(" Oops🤨 you got this Error 🔴  {e}"); continue}
            },
            _ => println!("Oops!!🙊🙊  Invalid Input \n");
        }

        let mut is_continue = get_user_input("Can we Continue (Y) or press any key to quit");

        if is_continue.trim().to_lowercase() != "y"{
            println!(" Thank you for using out calculator!! ");
            break;
        }
    }
}

fn handle_operation(option:u8) -> Result<i32 ,ErrorKind>{
    match option {
        1 => handle_addtion(),
        2 => handle_subtraction(),
        3 => handle_multiplication(),
        4 => handle_division(),
        _ => Err(InvalidData)
    }
}



pub fn handle_addtion() -> Result<i32,ErrorKind>{
      todo!()

}

pub fn handle_subtraction() ->Result<i32,ErrorKind>{
    todo!()
}
pub fn handle_multiplication() -> Result<i32,ErrorKind>{
    todo!()
}

pub fn handle_division() -> Result<i32,ErrorKind>{
    todo!()
}