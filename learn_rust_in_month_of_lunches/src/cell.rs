use std::{borrow::BorrowMut, cell::{Cell, RefCell}, sync::{Mutex, RwLock}};

#[derive(Debug)]
struct Person{
    name:String,
    profession:String,
    isMarried:Cell<bool>
}

impl Person {
    fn set_as_married(&self ){
        self.isMarried.set(true);
    }
}

pub fn test_cell(){
    let p1 = Person{
        name:"P1".to_string(),
        profession:"Software Enginner".to_string(),
        isMarried:Cell::new(false)
    };

    p1.set_as_married();
    println!(" person 1: {p1:#?} ,");
}

pub fn test_ref_Cell(){
    let ref_cell = RefCell::new("#6567");
    let mut mut_cell = ref_cell.borrow_mut();
    *mut_cell = " Hey unnai dhane!!";
    println!("{mut_cell:?}");
    drop(mut_cell);
    println!("{ref_cell:?}");
}

pub fn test_deadLock_mutex(){
    let mutex = Mutex::new(8);
    let mutex_changer = mutex.lock().expect("Unable to get Lock");
    let another_mutex_Changer = mutex.lock().expect("Does not able to get lock");
    println!(" This line never prints!!");

}

pub fn test_deadLock_rwlock(){
    let rw_lock = RwLock::new(9);
    let mut write_lock  = rw_lock.write().unwrap();
    let mut another_write_lock = rw_lock.write().unwrap();
    println!("This line nerver pprints!!");
}