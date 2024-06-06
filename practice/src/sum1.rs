pub fn sum_using_iter(){
    let sum1:i32 = (0..21).sum();
    let arr = [1,2,3,4,5,6];
    let sum2:i32 = arr.into_iter().sum();
    println!("{} {}",sum1,sum2);
    println!("{:?} ",arr);
}