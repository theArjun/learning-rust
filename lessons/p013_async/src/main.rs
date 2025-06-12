async fn make_request(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let response: serde_json::Value = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    Ok(response)
}

async fn get_nepse_index() {
    let api_url: &str = "https://data.nepse.bot/todays-index/NEPSE";
    let my_repsonse: Result<serde_json::Value, reqwest::Error> = make_request(api_url).await;
    match my_repsonse {
        Ok(r) => {
            dbg!(r);
        }
        Err(_) => {
            panic!("Failed to make request");
        }
    }
}

#[tokio::main]
async fn main() {
    get_nepse_index().await;
}
