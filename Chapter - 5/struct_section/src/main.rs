
struct User{
    email:String,
    name:String,
}

impl User{
    fn user_name_with_mail(&self) ->String{
        let mut s = String::from(self.name.to_string());
        s.push(' ');
        s.push_str(&self.email.to_string());
        s        
    }
}
fn main() {
    println!("Hello, world!");
    let user1:User =User { email:"dummy@gmail.com".to_string(), name: "dummy".to_string() };
    println!("User1 name is {} and email is {} ",user1.name,user1.email);

    println!("USer name with mail id {} ",user1.user_name_with_mail());

}
