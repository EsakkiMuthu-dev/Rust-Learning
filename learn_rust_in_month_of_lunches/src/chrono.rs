use chrono::{naive::{NaiveDate,NaiveDateTime}, NaiveTime};

pub fn test_chrono(){
    println!("{:#?}",NaiveDate::from_isoywd_opt(2015, 3, chrono::Weekday::Fri));
    println!("{:#?}",NaiveDateTime::from_timestamp_millis(123));
    println!("{:#?}",NaiveTime::from_hms_opt(12, 32, 37));


    // testing  thread resource usage
    for _ in 0..1000000000 {
        std::thread::spawn(|| {});
    }
}