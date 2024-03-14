fn main() {
     let is_rain:bool = true;
     let weather = if is_rain {"Rainy weather"} else {"Not Rainy Weather"};
     println!("{weather}");

     if is_rain
     {
        println!("Go with umbrella..");
     }else{
        println!("Go  and njoy.....");
     }

}
