use std::io;

fn main() {
    // factorial of given number : ->  5! -> 1*2*3*4*5
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect(" Not able to recieve the number");

    let number:u32 = number
                        .trim()
                        .parse()
                        .expect("Unable to parse the number");
    let mut factorial:u32 = 1;
    for i in 2..=number{
        factorial *= i;
    }

    println!("The Factorial of Given number {number} is {factorial}");

}
