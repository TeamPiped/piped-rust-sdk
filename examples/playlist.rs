use piped::PipedClient;
use reqwest::Client;

const INSTANCE: &'static str = "https://pipedapi.kavin.rocks";

#[tokio::main]
async fn main() {
    let httpclient = Client::new();

    let client = PipedClient::new(&httpclient, INSTANCE);

    let playlist = client
        .playlist_from_id("PLQSoWXSpjA38FIQCvwnVNPlGPVA63WTD8".to_string())
        .await
        .unwrap();

    println!("{:#?}", playlist);
}
