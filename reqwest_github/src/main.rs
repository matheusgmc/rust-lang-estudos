use reqwest::{header, Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: i32,
    login: String,
    avatar_url: String,
    name: String,
    public_repos: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/users/{owner}",owner = "matheusgmc");
    let router = Client::new();
    let response = router
        .get(request_url)
        .header(header::USER_AGENT, "reqwest")
        .send()
        .await?;
    let user = response.json::<User>().await?;


    println!("{:?}", user);
    Ok(())
}
