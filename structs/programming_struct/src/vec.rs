pub fn test_vec_i32(){
    let mut v:Vec<i32> = Vec::new();
    v.push(18);
    v.push(14);
    v.push(121);
    v.push(1221);
    println!(" Length of Vector {}",v.len());
    println!(" Capacity of Vector {}",v.capacity());
    println!(" vectoed {:?}",&(v).as_slice()[2..4]);
    println!("{:?}",v);
    v.retain(| i : &i32| if i%2==0 {true} else {false});
    println!("{:?}",v);
}