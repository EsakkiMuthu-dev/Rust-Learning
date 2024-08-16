pub mod print_bharathi{
    pub struct Bharathi{
        name:String,
        pub no_of_times_print:i32
    }

    impl Bharathi{
        pub fn new(no_of_times_print:i32) -> Self{
            Self{
                name:"Bharathi Ravi".to_string(),
                no_of_times_print : no_of_times_print
            }
        }
        pub fn print_mine(&self)
        {
            for i in 0..self.no_of_times_print{
                println!(" My {}",self.name);
            } 
        }
    }
}


pub mod country{
    fn print_country(country:&str){
        println!(" We are in the Country :  {country}. ");
    }
    pub mod state{
        fn print_state(state:&str){
            println!(" We are in the state :  {state}");
        }

        pub mod city{
            use super::print_state;

            pub fn print_city(country:&str,state:&str,city:&str)
            {
                super::super::print_country(country);
                crate::demo_mod::country::print_country(country);

                super::print_state(state);
                print_state(state);
                crate::demo_mod::country::state::print_state(state);

                println!(" We are in this city : {city}");
                println!(" We are in this city: {city}");
            }
        }
    }
}