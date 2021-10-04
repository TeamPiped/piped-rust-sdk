use piped::PipedClient;
use reqwest::Client;

const INSTANCE: &'static str = "https://pipedapi.kavin.rocks";

#[tokio::main]
async fn main() {
    let httpclient = Client::new();

    let client = PipedClient::new(&httpclient, INSTANCE);

    let video = client
        .video_from_id("__hYx6ZzFbQ".to_string())
        .await
        .unwrap();

    println!("{:#?}", video);
}
