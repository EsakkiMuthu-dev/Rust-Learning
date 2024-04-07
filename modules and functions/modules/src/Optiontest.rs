use crate::closures::some_fun;

pub fn give_option() -> Option<String>{
    let opt:Option<String> = Some("Hello".to_string());
    opt
}

pub fn give_option_int() -> Option<u32>{
    let opt : Option<u32> = None;
    opt
}

pub fn give_option_show() -> Option<show>{
    let mut opt : Option<show> = None;
    opt = Some(show::BreakingBad);
    opt
}

#[derive(Debug)]
pub enum show {
    GameOfThornes,
    BreakingBad,
    StrangerThings
}

impl ToString for show{
    fn to_string(&self) -> String {
        match &self{
            show::BreakingBad => "Breaking Bad",
            show::GameOfThornes => "Game Of Thornes",
            show::StrangerThings =>  "Stranger Things"
        }.to_string()
    }
}