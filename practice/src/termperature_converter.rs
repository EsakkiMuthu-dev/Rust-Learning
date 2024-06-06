use std::{i16, i8, io::stdin, option, primitive};
pub fn run()
{
    // get user input
    println!("<-------------- Welcome To Temperature Converter Program ------------>");
    loop {
        println!();
        println!(" Happy to help you using rust!🦀");
        println!("Options: ");
        println!("Press 1  to convert from celsius to faranheit");
        println!("Press 2  to convert from faranheit to celsisus");
        let mut option =  get_user_input(" Choose one of the option: ").trim().parse::<u8>();

       let option = match option{
                    Ok(1) => {
                        let result = convert_c_to_f();
                        println!(" {:.2} f",result);
                    } ,
                    Ok(2) =>{
                        let result = convert_f_to_c();
                        println!("{:.2} c",result);
                    }
                    _ =>{
                        println!("Invalid Option\n");
                        continue
                    }
       };

       let mut choice = get_user_input("Do you want to continue (y) or press any key to quit");
       if choice.trim().to_lowercase() != "y"{
            break;
       }
    }

}

// convert celisus to faranheit
fn convert_c_to_f() -> f32{
    let mut celsisus = get_user_input("\n Enter  the value of celesisus: ").trim().parse::<f32>();
    let celsisus = celsisus.expect("Invalid input for celsius");
    ((celsisus * 9.0)/5.0 )+32.0
}

// convert faranheit to celsisus
fn convert_f_to_c() -> f32{
    println!(" \n Enter the value of faranheit: ");
    let mut faranheit = String::new();
    stdin().read_line(&mut faranheit).expect("unable to get user input");
    let faranheit = match faranheit.trim().parse::<f32>(){
        Ok(n)=>n,
        Err(err) => {panic!(" invalid input for faranheit")}
    };
    ((faranheit - 32.0 ) * 5.0)/9.0
}

// helper function to get user input
fn get_user_input(prompt : &str) -> String{
    println!("{prompt}");
    let mut input = String:: new();
    stdin().read_line(&mut input).expect("Unable to get user input");
    input
}