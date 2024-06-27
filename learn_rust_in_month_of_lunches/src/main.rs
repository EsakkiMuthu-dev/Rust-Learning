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
}
