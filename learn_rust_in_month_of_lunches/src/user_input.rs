
pub fn test_user_input(){
    let mut input_string = String::new();
    println!(" \n Print X to Escape \n");

    while input_string.trim() != "X"{
        input_string.clear();
        std::io::stdin().read_line(&mut input_string);
        println!(" You entered {input_string:?}");
    }
}
pub fn test_env_vars(){
    for (key,value) in std::env::vars(){
        println!("{key} : {value}");
    }
}