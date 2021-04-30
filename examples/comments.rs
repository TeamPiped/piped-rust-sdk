use piped::piped::PipedClient;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() {
    let httpclient = ClientBuilder::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0")
        .build()
        .unwrap();

    let instance = "https://pipedapi.kavin.rocks".to_string();

    let client = PipedClient::new(httpclient, instance);

    let comments = client
        .get_comments_from_id("__hYx6ZzFbQ".to_string())
        .await
        .unwrap();

    println!("{:?}", comments);
}
