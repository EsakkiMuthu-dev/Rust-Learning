use std::mem;

#[derive(Debug)]
struct Demo{
    name : String
}

impl From<String> for Demo {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}
impl  Demo{
    fn change_value(&mut self, new_name:String)
    {
        let former = mem::replace(&mut self.name, new_name.clone());
        println!(" {former} now changed to {new_name}");
    }
}

pub fn test_into(){
    let str = String::from("hey!!");
    let mut demo: Demo = str.into();
    println!("{:?}",demo);
    demo.change_value("Hello".to_string());
    println!("{:#?}",demo);
    dbg!(vec![1,2,4]);
}