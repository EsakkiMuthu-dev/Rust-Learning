use std::{error, io::stdout, iter::Sum, path::Display};

use crossterm::{event::{read, Event, KeyCode, KeyEvent, KeyEventKind}, execute, terminal::Clear};
use reqwest::blocking::get;
use serde::{de::Error, Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug,Error)]
enum AppError{
    #[error("Http Request Failed to {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serde Failed to {0}")]
    JSON(#[from] serde_json::Error),

    #[error("Unknown error Failed to occured")]
    Unkown,
}

#[derive(serde::Serialize,serde::Deserialize,Debug,Default)]
struct Summary{
    title : String,
    description:String,
    extract:String
}

#[derive(Default)]
struct App{
    current_summary:Summary,
    user_input:String,
}
impl  App{
    fn get_article(&mut self)-> Result<(),AppError>{
        let data = get(format!("{URL}/{}",self.user_input))?;
        self.user_input.clear();
        let text = data.text()?;
        self.current_summary = serde_json::from_str(&text)?;
        println!("{self}");
        Ok(())
    }
}

impl std::fmt::Display for App{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(
        f,"
            Searching :  {0}
            <------------------->
            Title : {1}
            Description:{2}
            summary:{3}
        
        ",self.user_input,self.current_summary.title,self.current_summary.description,self.current_summary.extract)
    } 
}


const URL: &str = "https://en.wikipedia.org/api/rest_v1/page/summary";
fn main() {
    let mut app = App::default();
    loop {
        
        if let Event::Key(key_event) = read().unwrap(){
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
                        app.get_article();
                    }
                    _ => {}
                }
            }
        }
    }
}