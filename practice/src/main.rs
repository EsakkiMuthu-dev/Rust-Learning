mod termperature_converter;
mod sum1;
mod simple_calculator;
mod helper;
mod trait_exercise;

fn main() {
     // sum1::sum_using_iter();
     // termperature_converter::run();
     // simple_calculator::run();
     trait_exercise::run();
     println!("Size of a char: {}", std::mem::size_of::<char>()); 
     println!("Size of a: {}", "a".len());
     println!("Size of ß: {}", "ß".len());
     println!("Size of 国: {}", "国".len());
     println!("Size of : {}", "".len());

}
