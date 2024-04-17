pub mod my_fs;
fn main() {
    println!("Hello, world!");
    my_fs::create_dir();
    my_fs::create_file();
    my_fs::delete_all();
    
}
