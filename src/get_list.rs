use reqwest::header::{Headers, Authorization};
use crate::token::Token;
use serde_json::{Value};

pub fn get_list(token: &Token) -> Vec<Value> {
    let endpoint = "https://api.twitter.com/1.1/favorites/list.json";
    let mut headers = Headers::new();
    headers.set(Authorization(format!("Bearer {}", token.access_token)));
    let client = reqwest::Client::new();
    println!("start");

    client
        .get(endpoint)
        .query(&[("screen_name", "id")])
        .headers(headers)
        .send()
        .unwrap()
        .json()
        .unwrap()
}
