
pub fn test_iterator()
{
    let vec = vec![1,2,3,4,4,5,6,7,8,9,0];
    let vec_2 = vec
                    .iter()
                    .skip(6)
                    .map(|x| x+3)
                    .collect::<Vec<i32>>();
    println!("{vec:?} {vec_2:?}",);
}