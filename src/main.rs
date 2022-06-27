use std::time::Duration;

use futures::future::join_all;
use reqwest::Client;
use tokio::time;

use crate::{database::influxdb, endpoint::MonitoringEndpoint};

mod database;
mod endpoint;

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
                    let response_time = monitoring_endpoint.response_time().await;
                    println!("{:?}", status);
                    println!("{:?}", response_time);

                    let influxdb_point = influxdb::generate_point(&monitoring_endpoint).await;
                    println!("{:?}", influxdb_point);
                } else {
                    println!("{:?}", monitoring_endpoint.error().await.unwrap());
                }
            }
        });
    });

    join_all(handles).await;
}
