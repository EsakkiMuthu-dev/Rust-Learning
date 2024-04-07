
pub mod sorry_word{
    pub fn join_wrds(first:&str, last: &str) ->String{
    let full_word = format!("{first} {last}");
    return full_word;
}
}

pub mod to_it_repeat{
    pub fn ask_sorry_with_sorry_wrd(wrd:&String)
    {
        let mut counter:u8 =0;
        loop{

            println!("{wrd}");
            counter+=1;
            if counter > 100{
                break;
            }
        }
    }
}