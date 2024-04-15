use std::{sync::Mutex, thread::scope};


pub fn test_mutex()
{
    let  score = Mutex::new(0u16);
    
    {
        let mut locked_data = match score.lock(){
        Ok(guard) => guard,
        Err(poisoned_err) => poisoned_err.into_inner()
        };    
        *locked_data += 12;
    }

    let thread1 = || {
        // accuquring lock
        println!(" Thread 1  accuquring mutex lock ");
        let mut locked_data =score.lock().unwrap();
        for i in 1..15{
            *locked_data += i;
            println!("Thread 1 adding {i}");
        }
    };
    let thread2 = || {
        // accuquring lock
        println!(" Thread 2  accuquring mutex lock ");
        let mut locked_data =score.lock().unwrap();
        for i in 1..15{
            *locked_data += i;
            println!("Thread 2 adding {i}");
        }
    };
    _ = scope(|s|{
        s.spawn(thread1);
        s.spawn(thread2);
    });
    println!("Final score is {}. ",score.lock().unwrap());

}