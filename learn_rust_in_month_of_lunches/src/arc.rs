use std::{sync::{Arc, Mutex}, thread::{JoinHandle, Thread}};


pub fn test_arc_with_two_threads()
{
   let my_number = Arc::new(Mutex::new(0));

   let cloned1 = Arc::clone(&my_number);
   let cloned2 = Arc::clone(&my_number);

  let join_handle = std::thread::spawn(move ||{
        for _ in 1..=10{
            *cloned1.lock().unwrap()+=1;
            println!("Thread 1 is Working");
        }
   });
   let join_handle2 = std::thread::spawn(move ||{
        for _ in 1..=10{
            *cloned2.lock().unwrap() +=1;
            println!(" Thread 2 is Working");
        }
   });   
   join_handle.join().unwrap();
   join_handle2.join().unwrap();
   println!("Mynumber  : {:?}",my_number);
}

pub fn test_arc_with_multiple_threads(){
    let my_number = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![];

    for _ in 1..=10{
        let cloned_number = Arc::clone(&my_number);
        let thread = std::thread::spawn(move ||{
            for _ in 1..=10{
                *cloned_number.lock().unwrap() +=1;
            }
        });
        handle_vec.push(thread);
    }
    handle_vec.into_iter().for_each(|handle| handle.join().expect(" Unable to Join the Spawned thread with the main thread"));
    println!("MyNumber : {my_number:?}");
}