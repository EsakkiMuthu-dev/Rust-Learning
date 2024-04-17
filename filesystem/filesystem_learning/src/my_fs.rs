use std::{fs::{self, File}, io::Write, path::Path};

pub fn create_dir()
{
    let path = "./data";
    if Path::new(path).exists()
    {
        println!(" is already created!!");
    }
    let created_dir = fs::create_dir(path);
    if created_dir.is_ok()
    {
        println!(" Is successfully created!!");
    }else{
        println!(" not able to create {:?}.",created_dir.err());
    }
}

pub fn create_file()
{
    let file1 = "./data/file1.txt";
    let file2 = "./data/file2.txt";
    let file3 = "./data/file3.txt";
    
    _ = fs::write(file1, "Hey What are yopu doing? 🙋🏻‍♂️");
    _ = fs::write(file2, "Rusty Boy🧒🧒 🦀  🙋🏻‍♂️");
    _ = fs::write(file3, "Rainy Simpsy Mo 🌨️");
}

pub fn delete_all()
{
    fs::remove_dir_all("./data");
}