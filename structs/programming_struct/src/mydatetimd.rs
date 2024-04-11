use std::{ops::Sub, time::{Duration,Instant}};

pub fn test_time()
{
    let dur1 = Duration::from_secs(10);
    let dur2 = Duration::from_millis(18500);
    println!("{:?}",dur1.checked_sub(dur2).unwrap_or_default());
    let now = Instant::now();
    std::thread::sleep(Duration::from_millis(2000));
    println!("{:?}",now.elapsed().as_millis());
}