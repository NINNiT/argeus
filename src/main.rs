use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use futures::{stream, StreamExt};
use request::get_request;
use reqwest::Client;
use tokio::{task, time};

mod request;

const PARALLEL_REQUESTS: usize = 2;
#[tokio::main]
async fn main() {
    let client = Client::new();
    let urls = vec!["https://reqres.in/api/users?page=2"; 2];

    urls.iter().for_each(|url| {});

    // let forever = tokio::spawn(async move {
    //     let mut interval = time::interval(Duration::from_secs(5));

    //     loop {
    //         let cloned_client = client.clone();
    //         interval.tick().await;
    //         let request = get_request(cloned_client, "https://reqres.in/api/users?page=2").await;
    //         println!("{:?}", request.status());
    //     }
    // });

    // forever.await.unwrap();
}
