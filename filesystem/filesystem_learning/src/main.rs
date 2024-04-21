pub mod my_args;
pub mod my_fs;
fn main() {
    println!("Hello, world!");
    my_fs::create_dir();
    my_fs::create_file();
    my_fs::delete_all();
    my_args::get_from_args();
    println!("{}",some().await);
}

async fn some()
{
    println!("hello");
    await 0
}
