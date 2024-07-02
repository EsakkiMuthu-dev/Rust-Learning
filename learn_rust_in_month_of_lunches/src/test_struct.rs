

//unit struct
#[derive(Debug)]
struct SomeStruct;

// Tuple Struct
#[derive(Debug)]
struct Song(String,String,SomeStruct);

// Real Struct
#[derive(Debug)]
struct Real{
    some_on:SomeStruct,
    song:Song
}

pub fn test_struct()
{
    let some = SomeStruct;
   let song = Song("Lyrics".to_string(),"Music".to_string(),some);
    let no = SomeStruct;
    let real_struct: Real = Real{some_on:no,song};
    println!("{:?}",real_struct);
}