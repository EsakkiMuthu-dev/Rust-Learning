use std::fmt::{Debug, Display};

pub fn test_generice_item<T:Debug>(item: T)
{
    println!(" Some item {item:?}");
}

pub fn display_and_compare<T,U>(statement:T , num1:U, num2:U) 
where
    T : Display,
    U : Display + PartialOrd
{
    println!("{statement} Is this {num1} is greater than {num2}? ");
    println!("{}",num1 > num2);
}

pub fn take_sixth_element_from_vec(vec : &Vec<i32>) -> Option<i32>
{
    if vec.len() > 6 
    {
    Some(vec[6])
    }
    else {
        None
    }
}