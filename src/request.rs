use std::sync::{Arc, Mutex};

use reqwest::{Client, Response, StatusCode};

pub async fn get_request(client: Client, url: &str) -> Response {
    let response = client.get(url).send().await.unwrap();
    response
    // assert_eq!(response.status(), StatusCode::OK);
}

// pub async fn post_request_with_body(url: &str, body: &str) -> Response {
//     let client = Client::new();
//     let response = client.post(url).body(body).send().await.unwrap();
//     response
//     // assert_eq!(response.status(), StatusCode::OK);
// }
