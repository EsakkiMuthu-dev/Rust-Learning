pub mod my_fs;
pub mod my_args;
fn main() {
    println!("Hello, world!");
    my_fs::create_dir();
    my_fs::create_file();
    my_fs::delete_all();
    my_args::get_from_args();
}
