use std::env::args;

pub fn get_from_args() {
    let arguments: Vec<String> = args().collect();
    println!(" Argumetnts are {:?} ", arguments)
}
