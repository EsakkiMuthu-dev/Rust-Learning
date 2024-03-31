enum Weather{
    Raining,
    Sunny
}
fn main() {
    println!("Hello, world!");
    let s = Weather:: Raining;
    match s {
       Weather::Sunny =>  println!("Today weather is sunny ☀️"),
       Weather::Raining => println!("Today weather is Raining🌧️")
    }
    
}
