use std::io::stdout;

use crossterm::{event::{read, Event, KeyCode, KeyEvent, KeyEventKind}, execute, terminal::Clear}; 

struct App{
    file_content:String,
    user_input:String
}

impl App {
    fn new() -> Result<Self,std::io::Error>{
        let file_content = std::fs::read_to_string("/Users/esakki-20378/Documents/GitHub/Rust-Learning/typing_tutor/src/typing.txt")?;
        Ok(Self{
            file_content,
            user_input:String::new()
        })
    }
}


fn main() -> Result<(),std::io::Error> {

    let mut app = App::new()?;
    loop {
        println!("{}",app.file_content);
        for (letter1,letter2) in app.file_content.chars().zip(app.user_input.chars()){
            if letter1 == letter2  {
                print!("{}",letter1);
            }
            else{
                print!("*");
            }
        }
        println!("_");
        if let Event::Key(key_event) = read().unwrap(){
            if key_event.kind == KeyEventKind::Press{
                match key_event.code {
                    KeyCode::Backspace =>{
                        app.user_input.pop();
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char(a) =>{
                        app.user_input.push(a);
                    }

                    KeyCode::Enter => {

                        // Lets Compare how and find the results
                        let result = app.user_input
                                                .chars()
                                                .zip(app.file_content.chars())
                                                .filter(|(a,b)| a == b)
                                                .count();

                        let total = app.file_content.chars().count();

                        println!(" You got {result} right out of {total} ! ");
                        return  Ok(());                      
                    }

                    _ => {}
                }
            }
            execute!(stdout(),Clear(crossterm::terminal::ClearType::All));
        }
    }
    Ok(())

}
