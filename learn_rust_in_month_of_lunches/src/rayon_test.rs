use std::time::Instant;

use rayon::prelude::*;
pub fn test_rayon()
{
    println!("Available parllelism On M1 Mac {:?} ",std::thread::available_parallelism());

    let mut  nums = vec![0;2000000000];
    println!(" Size of this vec {:?}",nums.len());
    let time_start_for_normal_iter = Instant::now();
    nums    
        .iter_mut()
        .enumerate()
        .for_each(|(index,number)| *number+=index+1);
    println!("Printing values from 2000 to 2006{:?}",&nums[2000..2006]);
    println!(" Time taken for normal Iter without parallelism is {:?}",time_start_for_normal_iter.elapsed());

    println!("\n <--------------------------------------------------------> \n");
    let time_start_for_rayon = Instant::now();
    nums
        .par_iter_mut()
        .enumerate()
        .for_each(|(index,number)| *number+=index+2);
    println!("Printing Values from 2000 to 2006 {:?}",&nums[2000..2006]);
    println!(" Time taken for Rayon Iter With parallelism is {:?} ",time_start_for_rayon.elapsed());
}