fn main() {
    println!("Hello, world!");
    let x:i32 =5;
    let y:i32 = 6;
    let sum:i32 = calculate_sum(x,y);
    println!("The sum of x and y are {sum}");
    make_copy(x);
    println!("The value of x is {x}");

    let s1 : &str = "Hey Bharathi !!";
    make_stringliteral_copy(s1);
    println!("The value of s is {s1}");

    let s2:String = String::from("Hey Bharathi !!");
    // make_string_copy(s2);
    // println!("The value of s is {s2}");
    
    //dsnjxnkjds

    let s :String = String::from("Hey Bharathi !!");
    let (s2,len)= calculate_string_length(s);
    println!("the string {s2} and length is {len}");

    let s3 = String::from("Bharthiee");
    let s3_length:usize = calculate_string_length_using_references(&s3);
    println!("The length of s3({s3}) is {s3_length}");

}
fn calculate_string_length(s:String) -> (String,usize)
{
    let length:usize =s.len();
    (s,length)
}

fn calculate_string_length_using_references(s:&String) -> usize{
    s.len()
}

fn make_string_copy(S :String){
    println!(
        "The value of string is {S}"
    );
}

fn make_stringliteral_copy(s :&str)
{
    println!("The value of s  is {s}");
}

fn calculate_sum( x:i32,y:i32) -> i32
{
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    let sum :i32 = x+y;
    return sum;
}

// copy vs move in rust primitives copy 
fn make_copy(x:i32){
    println!("The value of x is {x}");
}
