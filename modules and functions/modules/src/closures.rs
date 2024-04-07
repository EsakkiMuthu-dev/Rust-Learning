pub fn some_fun()
{
    let add_fn = | x:i32 , y:i32| println!("helo fom closures {} times.",x+y);
    add_fn(56,89);  

    let calaculate_sum = |x:i32,y:i32| {
        println!("I am going to calculate the result");
        x+y
    };
    
}

