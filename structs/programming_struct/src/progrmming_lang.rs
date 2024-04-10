#[derive(Debug)]
pub struct ProgrammingLanguage{
    pub name:String,
    pub  author:String,
    pub  year: u16,
    pub specailty:Specailty
}

#[derive(Debug)]
pub enum Specailty{
    MachineLearningAndAI,
    SystemProgramming,
    AllInAll,
    EnterpriseProgramming,
    MotherOfALL
}

impl ProgrammingLanguage {
    pub fn change_speciality(&mut self ,speciality:Specailty){
        self.specailty =speciality;
    }
    pub fn get_lang_name_with_author(&self) -> String{
        format!("{} {} ",self.name ,self.author)
    }
}