struct Dog{
    name:String,
    dog_type:String
}

struct Robot{
    name:String,
    robot_type:String
}

impl speak for Dog {
    fn speak(&self) {
        println!("Vow vow vow 🐕🦴")
    }
}
impl speak for Robot {
    fn speak(&self) {
        println!("Boom Boom Robot ah robot ah 🤖")
    }
}
trait speak {
    fn speak(&self);
}

pub fn run(){
    let dog:Dog = Dog{
        name:String::from("Doggy"),
        dog_type: String::from("Some  variant")
    };
    dog.speak();
}