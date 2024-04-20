use reqwest::blocking::{Client, ClientBuilder};
use reqwest::redirect::Policy;

fn main() {
    let http_client = Client::new();
    // let response = http_client.get("https://crates.io/crates/reqwest").send();
    // if response.is_ok() {
    //     println!("{:?}", response.ok().unwrap().bytes());
    // }
    let redirect_policy = Policy::limited(3);
    let http_client_redirect = ClientBuilder::new()
        .redirect(redirect_policy)
        .build()
        .unwrap();
    let resoponse = http_client_redirect
        .get("http://localhost:3000/tamiblasters")
        .send()
        .unwrap();
    println!("{:?}", resoponse.bytes());
}
