fn main() {
  let abc:char = '😇';
  println!("{abc} ");

  // tuples
  let tup :(i32,f64,i8) =(12,34.546,12);
  let(a,b,c) =tup;
  println!("{a} {b} {c} ");

  //arrays
  let arr :[i32;5]=[1,2,3,2,5];
  println!("{}",arr[0]);
  println!("{}",tup.0);
}
