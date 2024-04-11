pub mod progrmming_lang;
pub mod tratits;
pub mod vec;
pub mod myhashmap;
use progrmming_lang::{ProgrammingLanguage,Specailty};
fn main() {
    let langs =vec!["Rust","Python","C","C++"];
    let mut prog_langs:Vec<ProgrammingLanguage> = create_prog_lang(&langs);
    println!("Before Changing Specaility: ");
    pretty_print(&prog_langs);
    println!("After Changing Specaility: ");
    update_specaility(&mut prog_langs);
    pretty_print(&prog_langs);
    let mut rust = ProgrammingLanguage::create_prog_lang("Rust".to_string(), "Graydon Hoare".to_string(), 2015);
    rust.change_speciality(Specailty::SystemProgramming);
    println!("{:?}",rust);
    tratits::create_person();
    vec::test_vec_i32();
    myhashmap::test_map();
}

fn create_prog_lang(langs: &Vec<&str>) -> Vec<ProgrammingLanguage>{
    let mut prog_langs:Vec<ProgrammingLanguage>=Vec::new();
    let mut counter:i8=0;
    for lang in langs{
        prog_langs.push(ProgrammingLanguage{name:lang.to_string(),author:"Someone".to_string(),year:2000,specailty:Specailty::AllInAll});
        counter+=1;
    }
    prog_langs
}

fn update_specaility(prog_langs:&mut Vec<ProgrammingLanguage>){
    for lang in prog_langs{
        match lang.name.as_str(){
            "Rust" => lang.change_speciality(Specailty::SystemProgramming),
            "Python"=> lang.change_speciality(Specailty::MachineLearningAndAI),
            "C" => lang.change_speciality(Specailty::MotherOfALL),
            "C++" => lang.change_speciality(Specailty::AllInAll),
            _ => { lang.get_lang_name_with_author() ; ()}
        }
    }
}

fn pretty_print(prog_langs: &Vec<ProgrammingLanguage>){
    println!("<--------    🦀   --------->");
    for lang in prog_langs{
        println!("{:?}",lang);
    }
    println!("<--------    🦀   --------->");
    println!("");
}