use std::sync::Mutex;

pub fn test_scoped_thread()
{
    let mynumber = Mutex::new(0);
    std::thread::scope(|s|{
            s.spawn(||{
                for _ in 1..=10000{
                    *mynumber.lock().unwrap()+=1;
                } 
            });
    });
    println!("My number  from Scroped Threads : {mynumber:?}");
}

pub fn test_scoped_thread_with_multiples(){
    let mynumber = Mutex::new(0);
    let regular_number = 12;
    let mut mut_regular_number = 1;

    std::thread::scope(|s|{
        s.spawn(||{
            for _ in 1..=109{
                *mynumber.lock().unwrap()+=1;
                println!("Using Regular variable its fine {regular_number}");
                mut_regular_number+=1;
            }
        });

        s.spawn(||{
            for _ in 1..13{
                *mynumber.lock().unwrap()+=1;
                println!("Printing regular var from multiple thread is fine {regular_number}");
                // mut_regular_number+=1;
            }
        });
    });

    println!("My number Value {mynumber:?}");
}