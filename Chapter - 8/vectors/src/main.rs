fn main() {
    //Vector Creation

    let a =[1,2,3,4,5];
    let mut v:Vec<i32> = Vec::new();
    v.push(3);
    v.push(5);
    v.push(4);

    println!(" Vector V1 is {:?}",v);

    let mut vec2 = vec![1,32,2,42];
    
    println!(" Vector V1 is {:?}",vec2);

    // access each element

    for i in  & mut vec2{
        println!("{i}");
        *i+=10;
        println!("{i}");
    }
}
