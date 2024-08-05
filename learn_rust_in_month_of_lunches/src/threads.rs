use std::thread::JoinHandle;

pub fn test_thread()
{
    let mut  handles :Vec<JoinHandle<()>> = vec![];
    for i in 0..10{
        println!();
        let handle = std::thread::spawn(move ||{
            println!(" I am From thread {i} and printing on console")
        });
        println!("Thread {i} started");
        println!();
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

}

pub fn test_fn()
{
    let string = " Hey i  Will be  printed by closuer ";
    let my_clouser_fn = || println!("{string}");
    my_clouser_fn();
    my_clouser_fn();
}

pub fn test_fn_mut(){
    let mut string = String::from("I am going to change this string by using closure ");
    let mut  my_closure_fn_mut = ||{
            string.push_str(" now changed!!");
            println!("{string}");
    };
    my_closure_fn_mut();
    my_closure_fn_mut();
}

pub fn test_fn_once(){
    let my_vec = vec![1,2,3,4,5];
    let my_closure = ||{
        my_vec.into_iter().for_each(|el| println!("{el}"))
    };
    my_closure();
}