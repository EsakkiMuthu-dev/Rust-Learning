use std::borrow::Cow;

pub fn modulo_8(num:i32) -> Cow< 'static ,str>{
    match num%8 {
        0 => "Remainder is 0".into(),
        1 => " Remainder is 1".into(),
        _ => format!("Hey i am having different remainder than 0 and 1").into()
    }
}

pub fn test_cow(){
    for num in 1..=10{
        match modulo_8(num){
            Cow::Borrowed(message) => println!(" This is the borrwed  message {message} . input {num}"),
            Cow::Owned(message) => println!("This is an owned message {message} , input {num} ")
        }
    }
}