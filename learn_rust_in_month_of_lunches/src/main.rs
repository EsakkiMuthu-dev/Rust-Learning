use std::char;
mod test_struct;
fn main() {
   // let float = 12.0;
   // {
   //  let float_two = 34.0;
   // }
   // println!("Add to floats {}",float+float_two);
   // let my_number = {
   //    let second_number = 8;
   //    second_number+9;
   // };
   // println!(" My number is {:?}",my_number);
   // println!(" The smallest of  u128  is {} and the largest of u128  is {}",u128::MIN , u128::MAX);
   let mut my_number = 12;
   let num = &mut my_number;
   *num+=1;
   let mut word = String::from("Helllo");
   let mut_word = &mut word;
   mut_word.push_str("Some other String ".into());
   println!("My_number is {my_number}");
   println!(" My string changed to {word}");
   println!("INside \t \\ \\\\Quotes
   sopmwe
   ajks");
   println!(r#" inside \ posu \\\\\\"#);
   println!("{:?}",b"This is something converted to binary");
   let num = 20;
   let num_ref = &num;
   println!("{:p}",num_ref);

   let father_name = "Esakki";
   let son_name = "Esakki Muthu"; 
   let family_name = "Esakki";
   println!("This is {1} {2}, son of {0} {2}.",
    father_name, son_name, family_name);

    let mut arr:[u8;5] = [0;5] ;
    arr[0] = 4;
    does_nothing();

    let tuple = ("hey","hello",45,'5',vec![12,67]);
    let(_,_,_,char_var,vec_var) = tuple;
    println!("{char_var:?}, {vec_var:?}");

    test_struct::test_struct();
}

fn does_nothing() -> u8
{
   println!("This Method does Nothing");
   78
}
