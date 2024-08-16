#[test]
pub fn two_is_even(){
    std::env::set_var("RUST_BACKTRACE", "1");
   println!(" {:?} " ,std::env::var("RUST_BACKTRACE"));
    assert_eq!(0,2%2);
}

pub fn returns_two() -> i8{
    2
}

pub fn returns_six() -> i8{
    returns_two()+4
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_returns_two()
    {
        assert_eq!(2,returns_two())
    }

    #[test]
    fn test_returns_six(){
        assert_eq!(6,returns_six())
    }

}