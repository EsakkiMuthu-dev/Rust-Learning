use crossterm::event::{read, Event, KeyCode, KeyEvent}; 
fn main() {

    let file_content = std::fs::read_to_string("typing.txt");
    let typed_content = String::new();

    if let Event::Key(key_event) = read().unwrap(){
        match key_event.code {
            KeyCode::Backspace => typed_content.pop(),
            
            
        }
    }
}
