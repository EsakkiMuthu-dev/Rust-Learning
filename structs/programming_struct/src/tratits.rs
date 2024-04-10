#[derive(Debug)]
struct Person<PetType,PetType2>
where PetType : Animal+NotDangerous,
      PetType2 : Animal+Dangerous  
{
    name:String,
    pet:PetType,
    pet2 : PetType2
}

trait Animal{
    fn make_sound(&self) -> ();
}
trait NotDangerous {
    
}

trait Dangerous {
    
}


#[derive(Debug)]
struct Cat{}


#[derive(Debug)]
struct Dog{}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Dog Barked");
    }
}
impl NotDangerous for Dog{}


#[derive(Debug)]
struct Tiger{}
impl Animal for Tiger {
    fn make_sound(&self) -> () {
        println!("Tiger Roared!!");
    }
}
impl Dangerous for Tiger {
    
}

pub fn create_person()
{
    let pet1=Dog{};
    let pet2=Tiger{};

    let person = Person{name:"Hey".to_string(),pet:pet1,pet2:pet2};
    person.pet.make_sound();
    person.pet2.make_sound();
}