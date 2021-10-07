use piped::PipedClient;
use reqwest::Client;

const INSTANCE: &'static str = "pipedapi.kavin.rocks";

#[tokio::main]
async fn main() {
    let httpclient = Client::new();

    let client = PipedClient::new(&httpclient, INSTANCE);

    let suggestions = client.search_channel("techlore").await.unwrap();

    println!("{:#?}", suggestions);
}
