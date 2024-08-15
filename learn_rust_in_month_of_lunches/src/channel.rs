use std::sync::mpsc::channel;

pub fn test_channel()
{
    let (sender,reciever) = channel();
    sender.send(5).unwrap();
    println!("{}",reciever.recv().unwrap());
}

pub fn test_channel_with_threads()
{
    let(sender,receiver) = channel();
    let sender_clone = sender.clone();
    std::thread::spawn(move ||{
        sender_clone.send(" Sending this &str from thread 1");
    });
    let sender_clone = sender.clone();
    std::thread::spawn(move ||{
        sender_clone.send("Sending another &str from thread 2");
    });

    while let Ok(res) = receiver.recv(){
        println!("received : {res}");
    };
}