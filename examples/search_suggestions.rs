use piped::PipedClient;
use reqwest::Client;

const INSTANCE: &'static str = "https://pipedapi.kavin.rocks";

#[tokio::main]
async fn main() {
    let httpclient = Client::new();

    let client = PipedClient::new(&httpclient, instance);

    let suggestions = client
        .search_suggestions("techlore".to_string())
        .await
        .unwrap();

    println!("{:#?}", suggestions);
}
