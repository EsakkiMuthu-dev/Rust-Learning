use std::io::stdout;

use crossterm::{event::{read, Event, KeyCode, KeyEvent, KeyEventKind}, execute, terminal::Clear};
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct App{
    user_input:String
}
impl  App{
    fn new() -> Self{
        Self{
            user_input : String::new()
        }
    }
}

#[derive(serde::Serialize,serde::Deserialize,Debug,Default)]
struct Summary{
    title : String,
    description:String,
    extract:String
}
const URL: &str = "https://en.wikipedia.org/api/rest_v1/page/summary";
fn main() {
    let mut app = App::new();
    loop {
        
        if let Event::Key(key_event) = read().unwrap(){
            execute!(stdout(),Clear(crossterm::terminal::ClearType::All));
            if key_event.kind == KeyEventKind::Press{
                match key_event.code {
                    KeyCode::Backspace =>{
                        app.user_input.pop();
                    } ,
                    KeyCode::Esc => {
                        break;
                    },
                    KeyCode::Char(ch)=>{
                        app.user_input.push(ch);
                    }
                    KeyCode::Enter =>{
                        let data = get(format!("{URL}/{}",app.user_input)).unwrap();
                        let text = data.text().unwrap();
                        let summary : Summary = serde_json::from_str(&text).unwrap();
                        println!("{:#?} \n \n \n",summary);
                    }
                    _ => {}
                }
            }
        }
    }
}


