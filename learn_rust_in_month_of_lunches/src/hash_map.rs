use std::collections::HashMap;
use std::collections::BTreeMap;
pub fn create_t20_map()
{
    let mut team_map = BTreeMap::new();
    team_map.insert("India", "India !! India!! -> Winners");
    team_map.insert("South Africa" , "Soth Africa ! !!!-> Runers");
    team_map.insert("Australia", "Semi Finalist");
    
    for (team , value) in team_map
    {
        println!("Team is {team} , They become {value}");
    }
}

pub fn reversed_string(input : &str) -> String{
    let mut reversed = String::new();
    let chars: Vec<char> = input.chars().collect();
    for  i  in (0..=reversed.len()).rev(){
        reversed.push(chars[i]);
    }
    reversed
}