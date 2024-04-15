use std::{sync::mpsc::{self, Receiver, Sender}, thread::{self, spawn}, time::Duration};
pub fn test_mpsc()
{
    //mpsc -> multiple produce single consumer
   let (sender , receiver) = mpsc::channel::<String>();

    let producer_thread =|i:u32,sender:Sender<String>|{
        for n in 1..=i{
            println!(" I am producing {n} message ");
            sender.send("Hey i am producing messages...!!".to_string());
            thread::sleep(Duration::from_millis(327));
        }
    };

    let consumer_thread = |receiver:Receiver<String>|{
        let mut err_count=0u8;
        let mut processed_msgs =0u32;
        loop{
            let message = receiver.recv_timeout(Duration::from_millis(400));
            if(message.is_ok())
            {
                processed_msgs +=1;
                println!(" Received Message from producer : {0} . processed message count: {1}",message.unwrap(),processed_msgs);
                err_count = 0;
            }else{
                err_count+=1;
                println!(" Occured error while consuming!!");
            }
            if(err_count > 10)
            {
                println!(" Producer is not producing messages . so we are stopping consuming process!!");
                break;
            }
        }
    };

    spawn(move ||{
         producer_thread(100 , sender.clone())
});
    spawn(move ||{
         consumer_thread(receiver)
});


}
    