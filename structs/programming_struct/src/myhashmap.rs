use std::collections::HashMap;

pub fn test_map()
{
    let mut maps:HashMap<String,i32> = HashMap::new();
    maps.insert("Hey".to_string(), 2);
    maps.insert("Hii".to_string(), 4);

    for map in maps{
        println!("Key {} Value {} ",map.0 ,map.1);
    } 
}