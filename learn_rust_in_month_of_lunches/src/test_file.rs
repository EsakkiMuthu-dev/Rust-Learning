use std::{fs::{read_to_string, File}, io::{Read, Write}};


pub fn test_file(){
    let mut file = File::options().append(true).create(true).write(true).open("Demo.txt").unwrap();
    file.write_all(b" \n \n \t  Hey i am Writing this to the File using Rust \n ");

    println!("{}",read_to_string("Demo.txt").unwrap());
}