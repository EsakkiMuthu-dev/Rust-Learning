mod test_struct;
mod animal_enum;
mod generics;
mod hash_map;
mod hash_set;
mod throw_question;
mod trait_example;
mod iterator;
mod closures;
mod map_method;
mod life_time;
mod cell;
mod cow;
mod rc_exmaple;
mod threads;
mod arc;
mod scoped_threads;
mod channel;
mod demo_mod;
use demo_mod::print_bharathi::Bharathi;
mod calc;
mod builder_pattern;
mod deref_test;
mod serde;
mod chrono;
mod rayon_test;
mod user_input;
mod test_file;
mod test;
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
   test_struct::casting_enum_as_int();
   animal_enum::test_enum();
   generics::test_generice_item(12);
   generics::display_and_compare("Hey, !! ", "hey12", "zabc24");
   let vec = vec![12,23,343,23,1,3,4,2];
   println!("Sixth elemnt in this {vec:?} is : {:?}",generics::take_sixth_element_from_vec(&vec));
   hash_map::create_t20_map();
   hash_set::try_hashset();
   let v = vec!["1", "hello","Deven","8"];
   for n in v{
      let result = throw_question::parse_and_log(n);
      println!(" Result : {result:?}");
   }
   trait_example::test_trait();
   iterator::test_iterator();
   closures::test_closures();
   map_method::test_map_method();
   println!("{}",life_time::print_str());
   cell::test_cell();
   cell::test_ref_Cell();
   // cell::test_deadLock_mutex();
   // cell::test_deadLock_rwlock();
   cow::test_cow();
   rc_exmaple::test_city();

   std::thread::spawn(||{
      println!(" I am A thread pringintg on console! from main!!");
   });
   threads::test_thread();
   threads::test_fn();
   threads::test_fn_mut();
   threads::test_fn_once();
   closures::test_fnonce();
   closures::test_fn();

   closures::test_closures_as_args();
   arc::test_arc_with_two_threads();
   arc::test_arc_with_multiple_threads();
   scoped_threads::test_scoped_thread();
   scoped_threads::test_scoped_thread_with_multiples();
   channel::test_channel();
   // channel::test_channel_with_threads();
   let bharu = Bharathi::new(12);
   bharu.print_mine();
   demo_mod::country::state::city::print_city("India", 
   "Tamil Nadu", "MelaSevaL");
   deref_test::test_deref_and_deref_mut();
   serde::test_serde();
   // chrono::test_chrono();
   // rayon_test::test_rayon();
   // user_input::test_user_input();
   user_input::test_env_vars();
   test_file::test_file();

}

fn does_nothing() -> u8
{

   println!("This Method does Nothing");
   78
}
