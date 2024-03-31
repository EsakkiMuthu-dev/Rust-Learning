
pub mod front_end;


mod back_end {
    // Example of calling the functions within a function
    pub fn setup() {
        //abslute path
        crate::front_end::setup::add_html();
        //relativepath
        super::front_end::setup::add_some_css();
    }
}

fn main() {
    // Call setup function from back_end module
    back_end::setup();
}
