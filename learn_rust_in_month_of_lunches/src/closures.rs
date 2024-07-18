pub fn test_closures(){
    let lambda = |x:i32| println!("{x}");
    lambda(12);
    lambda(2+8);

    let calculate_something = |x:i32,y:i32|{
        println!("The Addition of {x} and {y} is : {}",x+y);
        println!("The multiplication of {x} and {y} is: {} ",x*y);
        println!("The division of {x} and {y} is: {} ",x/y);
        println!("The subtraction of {x} and {y} is: {} ",x-y);
    };
    calculate_something(12,12);
    calculate_something(6,2);

    let chars = vec!['a','b','c'];
    chars
        .iter()
        .enumerate()
        .for_each(|(index,value)| { println!(" index {index}  and value is {value}")});

    let num_vec = vec![1,2,3,4,5];
    let num_vec_1 : Vec<i32> = num_vec
                            .iter()
                            .map(|num|{num*4})
                            .collect();
    println!("{num_vec:?}");
    println!("{num_vec_1:?}");

    let numbers ="140399923481800622623218009598281";
    for(index,num) in numbers.char_indices(){
        println!(" index {index} , value {num} {index}\t h ");
    }
}