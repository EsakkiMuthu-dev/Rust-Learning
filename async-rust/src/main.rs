use std::pin::Pin;

use futures::{select, FutureExt,pin_mut};
fn main() {
    let mut someThing = getSomething().fuse();
    let mut anything = getSomething().fuse();
    let mut noThing = getSomething().fuse();
    pin_mut!(someThing,anything,noThing);
    let result = smol::block_on(
        async{
            select!{
                    a=someThing => println!("The prcoess finsished {} ",a),
                    a=anything => println!("The prcoess finsished {} ",a),
                    a=noThing => println!("The prcoess finsished {} ",a),
            }
     
        }
    );
    
}

async fn getSomething() -> String {
    println!("Hello from async function!");
    return "Hurray".to_string();
}
