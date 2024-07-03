#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog
}


#[derive(Debug)]
struct Animal{
    age:u8,
    animal_type:AnimalType
}

impl Animal{
    fn new_cat(age:u8) -> Self{
            Self{
                age,
                animal_type:AnimalType::Cat
            }
    }
    
    fn new_dog(age:u8) -> Self{
        Self{
            age,
            animal_type:AnimalType::Dog
        }
    }

    fn check_animal_type(&self ){
        match self.animal_type {
            AnimalType::Cat => println!(" This is cat {self:?}"),
            AnimalType::Dog => println!(" This is dog {self:?}")
        }
    }

    fn change_to_cat(&mut self){
        self.animal_type =AnimalType::Cat;
    }

    fn change_to_dog(&mut self)
    {
        self.animal_type = AnimalType::Dog;
    }

}

pub fn test_enum()
{
    let mut animal = Animal::new_cat(24);
    animal.check_animal_type();
    animal.change_to_dog();
    animal.check_animal_type();
    animal.change_to_cat();
    animal.check_animal_type();

}