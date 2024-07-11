
struct SuperHero
{
    name:String,
}

struct HeisenBurg
{
    name:String,
}

trait Fly {
    fn fly(&self)
    {
        println!(" I am Flying!!");
    }
}
trait TagLike {
    fn who_am_i(&self);
}

impl Fly for SuperHero{}
impl TagLike for SuperHero {
    fn who_am_i(&self) {
        println!(" I am SuperMan")
    }
}

impl TagLike for HeisenBurg{
    fn who_am_i(&self) {
        println!(" I am not in Danger Skyler!! . I am the Danger");
    }
}
pub fn test_trait()
{
    let super_man = SuperHero{name:"IronMan".to_string()};
    super_man.fly();
    let proffessor = HeisenBurg{name:"Breaking Bad".to_string()};
    proffessor.who_am_i();
}