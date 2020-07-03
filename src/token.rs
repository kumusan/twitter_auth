use serde::Deserialize;
use reqwest::header::{Headers, Authorization, ContentType};
use base64;

#[derive(Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
}

pub fn get_token() -> Token {
    let consumer_key = "";
    let consumer_secret = "";
    let auth = base64::encode(&format!("{}:{}", consumer_key, consumer_secret));
    let endpoint = "https://api.twitter.com/oauth2/token";
    let mut headers = Headers::new();
    headers.set(Authorization(format!("Basic {}", auth)));
    headers.set(ContentType::form_url_encoded());
    let client = reqwest::Client::new();
    client
        .post(endpoint)
        .body("grant_type=client_credentials")
        .headers(headers)
        .send()
        .unwrap()
        .json()
        .unwrap()
}
