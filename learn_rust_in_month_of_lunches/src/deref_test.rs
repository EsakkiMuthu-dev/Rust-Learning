use std::ops::{Deref, DerefMut};

struct HoldsAnNumber(u8);

impl Deref for  HoldsAnNumber{
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HoldsAnNumber{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
pub fn test_deref_and_deref_mut(){
    let num = HoldsAnNumber(12);
    println!("{}",*num);
}
