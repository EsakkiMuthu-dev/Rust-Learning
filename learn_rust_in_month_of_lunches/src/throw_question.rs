use std::{num::ParseIntError};

pub fn parse_and_log(input : &str) -> Result<i32,ParseIntError>
{
    let  number = input.parse::<i32>()?;
    println!(" Numbers parsed Successfully into {number} ");
    Ok(number)
}