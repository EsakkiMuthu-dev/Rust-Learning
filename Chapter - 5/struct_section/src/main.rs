
struct User{
    email:String,
    name:String,
}
fn main() {
    println!("Hello, world!");
    let user1:User =User { email:"dummy@gmail.com".to_string(), name: "dummy".to_string() };
    println!("User1 name is {} and email is {} ",user1.name,user1.email);

}
