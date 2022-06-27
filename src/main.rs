use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use futures::{future::join_all, stream, StreamExt};
use reqwest::Client;
use tokio::{task, time};

use crate::request::MonitoringEndpoint;

mod request;

const PARALLEL_REQUESTS: usize = 2;
#[tokio::main]
async fn main() {
    let client = Client::new();
    // let urls = vec!["https://reqres.inuf/api/dsdsusers?page=2"; 2];
    let urls = vec!["https://reqres.in/api/users?page=2"; 2];

    // let mut monitoring_endpoint = MonitoringEndpoint::new(client.clone(), urls[0].to_string());
    // monitoring_endpoint.get_request().await;

    // if monitoring_endpoint.error().await.is_none() {
    //     let status = monitoring_endpoint.status().await;
    //     println!("{:?}", status)
    // } else {
    //     println!("{:?}", monitoring_endpoint.error().await.unwrap())
    // }

    let handles = urls.into_iter().map(|url| {
        let cloned_client = client.clone();
        return tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(5));
            let mut monitoring_endpoint = MonitoringEndpoint::new(cloned_client, url.to_string());

            loop {
                interval.tick().await;
                monitoring_endpoint.get_request().await;
                if monitoring_endpoint.error().await.is_none() {
                    let status = monitoring_endpoint.status().await;
                    println!("{:?}", status)
                } else {
                    println!("{:?}", monitoring_endpoint.error().await.unwrap())
                }
            }
        });
    });

    join_all(handles).await;
}
