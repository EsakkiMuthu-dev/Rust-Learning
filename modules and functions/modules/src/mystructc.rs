pub struct Person{
    pub first_name : String,
    pub last_name : String,
}

pub fn create_person() -> Person{
    let person = Person{
        first_name:"Maha".to_string(),
        last_name: "nandhini".to_string()
    };
    person
}