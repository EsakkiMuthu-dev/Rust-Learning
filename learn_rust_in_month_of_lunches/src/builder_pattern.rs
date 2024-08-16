
#[derive(Debug)]
struct Character{
    name:String,
    height:u16,
    weight:u16,
    is_alive:bool
}

#[derive(Debug)]
#[derive(Default)]
struct CharacterBuilder{
    name:String,
    height:u16,
    weight:u16,
    is_alive:bool
}

impl CharacterBuilder{
    fn height(mut self, height:u16) -> Self{
        self.height = height;
        self
    }

    fn weight(mut self , weight:u16) -> Self{
        self.weight = weight;
        self
    }

    fn name(mut self , name:String) -> Self{
        self.name = name;
        self
    }

    fn try_build(self) -> Result<Character,String>{
            if self.height < 200 && self.weight < 300 && !self.name.contains("fvdbjjdsvk"){
                Ok(Character{
                    name:self.name,
                    weight:self.weight,
                    height:self.height,
                    is_alive:true
                })
            }else{
                Err(" Can't create a charcter invalid value preset".to_string())
            }
    }

}


impl Default for Character{
    fn default() -> Self {
        Self{
            name:"Bharathi ".to_string(),
            height:156,
            weight:178,
            is_alive:true
        }
    }
}

pub fn test_builder(){
    let bharu = CharacterBuilder::default().height(173).weight(63).try_build().unwrap();
    
    println!("{:?} ",bharu)

}