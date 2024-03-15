fn main() {
    let mut number:u32 =1;
    let result = loop {
        number+=1;
        if number == 20{
            break number*100;
        }
    };

    'outer_loop: loop{
        loop {
            number+=2;
            if number == 3000{
                break 'outer_loop;
            }
        }
    }

    println!(" Result will be {result}");

    let arr =[1,2,3,4,5];
    let mut index:usize =0;   

    while index <5{
        println!("Value in index {index} is {} ",arr[index]);
        index +=1;
    }

    for element in arr{
        println!("{element}");
    }

    for i in (1..=32).rev(){
        println!("{i}");
    }
}
