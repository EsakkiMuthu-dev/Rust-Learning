use reqwest::Client;

pub async  fn test_reqwest()
{
    let client = Client::default();
    let res = client.get("https://pop.system76.com/").send().await.unwrap();
    println!("{:?}",res);
    println!("Some");
}

pub async  fn gives_two() -> u8{
    2
} 