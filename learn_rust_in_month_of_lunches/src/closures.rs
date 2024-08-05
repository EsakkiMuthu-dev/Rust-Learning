use std::collections::HashMap;

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

    let keys = vec![1,2,3,4,5];
    let values = vec!["hello","hey","hoii","some"];
    let nmap : HashMap<i32,&str> =  dbg!(keys.into_iter().zip(values.into_iter()).collect());

}

pub fn do_somethig_use_fnOnce<F>(f:F)
where F : FnOnce()
{
    f();
}

pub fn test_fnonce()
{
    let nums = vec![1,24,45,53,23,23];
    do_somethig_use_fnOnce(||{
        nums.iter().enumerate().for_each(|(index,el)|  {println!("Element is {el} and its index is {index}");});
    });
    println!("{nums:?}");
}

pub fn do_somethig_use_fn<F>(f:F)
where F:Fn()
{
    f();
}

pub fn test_fn(){
    let nums = vec![1,24,45,53,23,23];

    // do_somethig_use_fnOnce(||{
    //     for num in nums{
    //         println!("{num}");
    //     }
    // });
    do_somethig_use_fn(||{
        for num in &nums{
            println!("{num}");
        }
    });
}

pub fn takes_fn<F>(f:F)
where F:Fn(){
    f();
    f();
}

pub fn takes_fnMut<F>(mut f:F)
where F:FnMut(){
    f();
    f();
}

pub fn takes_fnOnce<F>(f:F)
where F:FnOnce(){
    f();
}

pub fn test_closures_as_args(){
    
    let mut my_string = String::from(" Hey !! i am This string");
    let string = String::from("hello !!");
    let fn_test = ||{
        println!("{string}")
    };
    takes_fn(fn_test);

    let fnmut_test = ||{
        my_string.push_str(" Changed !! ");
        println!("{my_string}")
    };
    takes_fnMut(fnmut_test);

    let fnonce_test = ||{
        println!("{string}");
        drop(string);
    };

    takes_fnOnce(fnonce_test);
}