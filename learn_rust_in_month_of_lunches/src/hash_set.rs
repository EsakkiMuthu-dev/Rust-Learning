use std::collections::HashSet;

pub fn try_hashset()
{
     let many_numbers = vec![
37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
46, 33, ];
    println!(" The length of many numbers vector is {}",many_numbers.len());
    let mut num_set = HashSet::new();
    for num in &many_numbers{
        num_set.insert(num);
    } 

    println!(" unique numbers  ion this many number vectors are {}",num_set.len());
    println!("Difference  is {}", many_numbers.len() - num_set.len());

    println!("{num_set:?}");
}
