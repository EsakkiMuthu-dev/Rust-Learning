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
}
