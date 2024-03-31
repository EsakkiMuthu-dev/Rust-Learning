use unicode_segmentation::UnicodeSegmentation;
fn main() {
    //Creating  a string

    let mut s = String::new();
    s.push_str("hello");
    let s = String::from("hey!!");
    let thirukural = String::from("அகர முதல எழுத்தெல்லாம் ஆதி
பகவன் முதற்றே உலகு
");

for i in thirukural.as_bytes(){
    println!("{i}");
}

for i in thirukural.chars(){
    println!("{i}");
}
for i in thirukural.graphemes(true)
{
    println!("{i}");
}
}
