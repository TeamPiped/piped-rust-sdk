use piped::PipedClient;
use reqwest::Client;

const INSTANCE: &'static str = "https://pipedapi.kavin.rocks";

#[tokio::main]
async fn main() {
    let httpclient = Client::new();

    let client = PipedClient::new(&httpclient, INSTANCE);

    let channel = client
        .channel_from_id("UCXuqSBlHAE6Xw-yeJA0Tunw".to_string())
        .await
        .unwrap();

    println!("{:#?}", channel);
}
