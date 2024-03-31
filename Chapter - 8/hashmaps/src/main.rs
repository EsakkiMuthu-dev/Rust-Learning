use std::collections::{hash_map, HashMap};

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

}
