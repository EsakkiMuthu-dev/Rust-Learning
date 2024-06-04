#![allow(unused)]
use std::io::{self, Read};
use rand::Rng;
use std::cmp::Ordering;


fn add_two_numbers(x: i32 , y:i32)
{
    println!("{x} + {y} = {}",x+y);
}
fn main(){
    add_two_numbers(12, 234);

    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }
    impl Day {
        fn is_weekend(&self ) -> bool{
            match self{
                Day::Saturday |  Day::Sunday =>true,
                _ => false
            }
        }
    }
    let today:Day = Day::Sunday;
    println!("is weekend ? . {}",today.is_weekend());
    // let str_1 = String::from("H ds d adsnklcas msclsaka s d s a");
    // let mut vec : Vec<char> = str_1.chars().collect();
    // vec.sort();
    // for letter in &vec{
    //     println!("{letter}");
    // }
    // vec.dedup();

    // for byte in str_1.bytes()
    // {
    //     println!("{byte}");
    // }

    // for letter in &vec{
    //     println!("{letter}")
    // }



    // let my_tuple : (u32,String,i8) =(466567,"Hello".to_string(),12);
    // println!(" First letter of tuple {}",my_tuple.0);
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
