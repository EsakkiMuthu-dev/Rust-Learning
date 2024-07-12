struct GivesOne;

impl Iterator for GivesOne{
    type Item = i32;

    fn next(&mut self) -> Option<i32>
    {
        Some(1)
    }
}


pub fn test_iterator()
{
    let vec = vec![1,2,3,4,4,5,6,7,8,9,0];
    let vec_2 = vec
                    .iter()
                    .skip(6)
                    .map(|x| x+3)
                    .collect::<Vec<i32>>();
    println!("{vec:?} {vec_2:?}",);
    let num:i32 = vec
                .iter()
                .filter(|x| **x%2==0)
                .sum();
    println!("{num} , {vec:?}");

    let mut num1 = GivesOne;
    println!("{:?} ",num1.next());
    println!("{:?} ",num1.next());
    println!("{:?} ",num1.next());
}