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
    MotherOfALL,
    Default
}

impl ProgrammingLanguage {
    pub fn change_speciality(&mut self ,speciality:Specailty){
        self.specailty =speciality;
    }
    pub fn get_lang_name_with_author(&self) -> String{
        format!("{} {} ",self.name ,self.author)
    }
    pub fn create_prog_lang(name:String,author:String,year:u16) -> ProgrammingLanguage{
        ProgrammingLanguage{name,author,year,specailty:Specailty::Default}
    }
}