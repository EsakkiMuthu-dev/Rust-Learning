#[derive(Debug)]
struct Demo{
    name : String
}

impl From<String> for Demo {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

pub fn test_into(){
    let str = String::from("hey!!");
    let demo: Demo = str.into();
    println!("{:?}",demo)
}