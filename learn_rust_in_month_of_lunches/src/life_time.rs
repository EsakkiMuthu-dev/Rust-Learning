pub fn print_str() -> &'static str{
    "hello"
}
struct Adeventurer<'a>{
    name: &'a str,
    hit_points : u32
}

impl Adeventurer<'_>{
    fn take_damage(&mut self){
        self.hit_points +=20;
        println!(" Adevturer name : {} and hit points : {}",self.name,self.hit_points);
    }
}

impl std::fmt::Display for Adeventurer<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " Hey , I am Adevnturer {} and my hit points are {}",self.name,self.hit_points)
    }
}