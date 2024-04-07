pub fn it_is_favourite_for_you(show_name: &str){
    match show_name {
        "game of thrones" => println!(" Not Today!!"),
        "breaking bad" => println!("I am the danger skyler!"),
         _ => println!("Yet to watch")
    }
}

pub fn some_match()
{
    let age =4;
    match age{
        1 => print!("hwllo"),
        _ => println!("hey")
    }
}

