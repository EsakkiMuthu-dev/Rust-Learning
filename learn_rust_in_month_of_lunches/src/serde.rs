
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize)]
struct User{
    name:String,
    id:u32,
    is_deleted:bool
}

#[derive(Debug,Serialize,Deserialize)]
struct UserRequest{
    id:u32,
    name:String,
}

impl From<UserRequest> for User{
    fn from(value: UserRequest) -> Self {
        Self { name: value.name, id: value.id, is_deleted: false}
    }
}

fn handle_request(request : &str){
    match serde_json::from_str::<UserRequest>(request) {
        Ok(good_request) =>{
                let new_user = User::from(good_request);
                println!("User {new_user:?}");
                println!("Serailized Back into JSON {:#?}",serde_json::to_string(&new_user));
        },
        Err(e) =>{
            println!(" Got an error {e:#?}");
        }
        
    }
   
}


pub fn test_serde(){
    let good_request = r#"
    {
        "name":"Hello",
        "id":12  
    }
    "#;

    let bad_request = r#"
    {
        "name":"hey1!",
        "idd":123
    }
    "#;

    handle_request(&good_request);
    handle_request(&bad_request);
}