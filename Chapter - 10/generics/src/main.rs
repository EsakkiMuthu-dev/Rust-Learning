fn main() {
    println!("Hello, world!");
    let vec_int = vec![1,2,3,4,5];
    let vec_float = vec![1.2,5.0,3.8,129.0];
    println!("The largest in this vector is {}",find_largest_int(&vec_int));
    println!("The largest in this vector is {:?}",find_largest_float(&vec_float));
    println!("The largest in this vector is {}",find_largest_any(vec_int));
    println!("The largest in this vector is {:?}",find_largest_any(vec_float));
}

fn find_largest_any<T: PartialOrd + Copy>(vec:Vec<T>) -> T{
    let mut largest = vec[0];
    for i in vec{
        if i >largest{
            largest = i;
        }
    }
    largest
}

fn find_largest_float(vec:&Vec<f64>) -> f64{
    let mut largest = vec[0];
    for i in vec{
        if *i > largest{
            largest = *i;
        }
    }
    largest
}

fn find_largest_int(vec:&Vec<i32>) -> i32{

    let mut largest = vec[0];

    for i in vec{
        if *i > largest{
            largest = *i;
        }
    }
    largest
}