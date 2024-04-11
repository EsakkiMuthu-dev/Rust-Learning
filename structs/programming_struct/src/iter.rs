pub fn test_iter()
{
    let fursits_list = vec!["Apple","Mango","Strawbwerry"];
    let  numbers_list = vec![2,3,4,1,32];
    let multiplied_list:Vec<i32>= numbers_list.iter().map(|number| number*2).collect();
    println!("{:?}",numbers_list);
    println!("{:?}",multiplied_list);
    numbers_list.iter().for_each(|number: &i32| println!("{number}"));
    for number in numbers_list.iter().chain(&multiplied_list)
    {
        println!("{}",number);
    }

    let some_expense_list = vec![("Random", 12),("Milk",2167), ("Diary milk",100)];
    let total_expense:u32= some_expense_list.iter().fold(0u32, |a:u32,expense| a+expense.1);
    println!("Expense  are : {:?}",total_expense);
}