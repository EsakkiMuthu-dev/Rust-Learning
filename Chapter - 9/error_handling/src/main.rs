use std::{fmt::Error, fs::File};
use std::io::ErrorKind;
fn main() {
    println!("Hello, world!");

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(f)=>f,
    //     Err(er)=> match er.kind() {
    //         ErrorKind::NotFound => match  File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(err) => panic!("Unable to create am file {err}"),
    //         },
    //         other_kind  => panic!("Got error {er}")
    //     }
    // };

    let f  = File::open("hello.txt").unwrap_or_else(|error|{
        if(error.kind() == ErrorKind::NotFound)
        {
            File::create( "hello.txt" ).unwrap_or_else(|err|{
                panic!("Cant able to create a file {:?}",err)
            })
        }else {
            panic!("Can't able to  open the file {:?}",error)
        }
    });
    

}
