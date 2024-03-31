mod web_dev {
     pub mod front_end {
        pub fn add_html() {
            println!("Adding HTML");
        }
        
        pub fn add_some_css() {
            println!("Adding CSS");
        }
    }
}

mod back_end {
    // Example of calling the functions within a function
    pub fn setup() {
        //abslute path
        crate::web_dev::front_end::add_html();
        //relativepath
        super::web_dev::front_end::add_some_css();
    }
}

fn main() {
    // Call setup function from back_end module
    back_end::setup();
}
