struct Animal{
    animal_type:String,
    is_run:bool
}

struct  Bird 
{
    bird_type:String,
    is_fly:bool
}

impl Eat for Bird {
    fn eating(&self) {
        println!("{}  is Eating now !",self.bird_type);
    }   
}

pub trait Eat {
    fn eating(&self);
}

fn main() {
    println!("Hello, world!");
    let b1 = Bird{
        bird_type:String::from("Crow"),
        is_fly:true
    };
    b1.eating();
}
