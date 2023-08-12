use piped::PipedClient;
use reqwest::Client;

const INSTANCE: &'static str = "https://pipedapi.kavin.rocks";
const CHANNELS: &[&str; 2] = &["UCXuqSBlHAE6Xw-yeJA0Tunw", "UCdBK94H6oZT2Q7l0-b0xmMg"];

#[tokio::main]
async fn main() {
    let httpclient = Client::new();

    let client = PipedClient::new(&httpclient, INSTANCE);

    let videos = client.bulk_feed(CHANNELS).await.unwrap();

    println!("{:#?}", videos);
}
