use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct IndexData {
    close: f64,
    date: String,
    difference: f64,
    high: f64,
    low: f64,
    open: f64,
    percent_change: f64,
    symbol: String,
    transaction: f64,
    turnover: f64,
    volume: f64,
}
async fn make_request(url: &str) -> Result<IndexData, reqwest::Error> {
    let response: IndexData = reqwest::get(url).await?.json::<IndexData>().await?;
    Ok(response)
}

async fn get_nepse_index() {
    let api_url: &str = "https://data.nepse.bot/todays-index/NEPSE";
    let my_repsonse: Result<IndexData, reqwest::Error> = make_request(api_url).await;
    match my_repsonse {
        Ok(r) => {
            println!("CLOSE : {}", r.close);
            println!("DATE : {}", r.date);
            println!("DIFFERENCE : {}", r.difference);
            println!("HIGH : {}", r.high);
            println!("LOW : {}", r.low);
            println!("OPEN : {}", r.open);
            println!("PERCENT CHANGE : {}", r.percent_change);
            println!("SYMBOL : {}", r.symbol);
            println!("TRANSACTION : {}", r.transaction);
            println!("TURNOVER : {}", r.turnover);
            println!("VOLUME : {}", r.volume);
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
