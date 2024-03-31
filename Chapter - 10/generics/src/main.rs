
#[derive(Debug)]
struct Point<T,U>{
    x:T,
    y:U
}
impl<S,U> Point<S,U> {
    fn mixed<V,W>(self,other:Point<V,W>) -> Point<S,W>{
         Point{
             x: self.x, 
             y:other.y}
    }
}

fn main() {
    println!("Hello, world!");
    let vec_int = vec![1,2,3,4,5];
    let vec_float = vec![1.2,5.0,3.8,129.0];
    println!("The largest in this vector is {}",find_largest_int(&vec_int));
    println!("The largest in this vector is {:?}",find_largest_float(&vec_float));
    println!("The largest in this vector is {}",find_largest_any(vec_int));
    println!("The largest in this vector is {:?}",find_largest_any(vec_float));

    let p = Point {x:1,y:3};
    let p1 =Point{ x:"hello",y:'s'};
    let p2 = p1.mixed(p);
    println!("The Mixed Point is {:?}",p2);


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