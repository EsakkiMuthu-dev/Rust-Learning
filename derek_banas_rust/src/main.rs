#![allow(unused)]
use std::io::{self, Read};
use rand::Rng;
use std::cmp::Ordering;
fn main(){

    // let arr_1 = [1,2,3,4,5,6,7,8,9];
    // let mut loop_idx : usize = 0;
    // loop {
    //     if arr_1[loop_idx] >= 9{
    //         println!(" Exiting loop");
    //         break;
    //     }
    //     if arr_1[loop_idx] % 2 != 0{
    //         println!(" Odd number {}", arr_1[loop_idx]);
    //     }
    //     loop_idx+=1;

    // }

    // let random_number = rand::thread_rng().gen_range(1..100);
    // println!("Random Number is {random_number}");
    
    // let age :u32 = 18;
    // let eligible_age : u32 = 18;
    // let can_vote = if age > 18 {true} else { false};
    // println!("{}",can_vote);

    // match age.cmp(&eligible_age)
    // {
    //     Ordering::Less => println!(" Not able to vote now . Wait for {} years",eligible_age - age),
    //     Ordering::Greater => println!("  you can vote"),
    //     Ordering::Equal =>println!(" lets start oruviral puratchi")
    // };

    // const ONE_MIL: u32 = 1_00_000;
    // println!("{}",u32::to_string(&ONE_MIL));
    // println!("{}",ONE_MIL)
    // println!("Hello, What's your name ? ");
    // let mut name = String::new();
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Oops ! Don't get ur name");
    // let greeting = " Nice to Meeting you";
    // println!("Hello {} ! {greeting}",name.trim_end());
}
