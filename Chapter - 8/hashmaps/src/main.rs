use std::collections::{ HashMap};

fn main() {
    println!("Hello, world!");
    let blue = String::from("blue");
    let yellow = String::from("yellow");
    let mut  map:HashMap<String, i32> = HashMap::new();
    map.insert(blue, 12);
    map.insert(yellow, 123);

    println!("{:?}",map);

   match map.get("blue"){
    Some(i) => println!("{i}"),
    _ =>  println!("Key not found")
   }
   let mut count_map :HashMap<&str,i32>= HashMap::new();
   let text = "hello world this is rust";
   for word in text.split_whitespace(){
      let count = count_map.entry(word).or_insert(0);
      *count+=1;
   }
   println!("{:?}",count_map);

}
