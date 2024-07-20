pub fn test_map_method(){
    let some_vec = vec!["1","67","hey","heello","8938389632738"];
    let mut num_char =vec![];

    for i in 1..some_vec.len()
    {
        num_char.push(
            some_vec
                    .get(i)
                    .and_then(|number| number.parse::<u32>().ok())
                    .and_then(|number| char::try_from(number).ok())
        )
    }
    
    for num in ["1","4","hellu","9"]
                                        .into_iter()
                                        .map(|number| number.parse::<u32>())
                                        .flatten(){
                                            println!("{num}");
                                        }


    println!("{num_char:?}")
}