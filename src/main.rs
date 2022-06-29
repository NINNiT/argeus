use std::{sync::Arc, time::Duration};

use futures::future::join_all;
use reqwest::Client;
use tokio::time;

use crate::{database::influxdb, endpoint::MonitoringEndpoint};

mod database;
mod endpoint;

#[tokio::main]
async fn main2() {
    let client = Client::new();
    let urls = vec!["https://reqres.in/api/users?page=2"; 2];

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

#[tokio::main]
async fn main() {
    let client = Client::new();
    let urls = vec!["https://reqres.in/api/users?page=2"; 2];

    let endpoints: Vec<MonitoringEndpoint> = urls
        .iter()
        .map(|url| {
            let cloned_client = client.clone();
            let endpoint = MonitoringEndpoint::new(cloned_client, url.to_string());
            return endpoint;
        })
        .collect();

    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;
        &endpoints.into_iter().for_each(|mut endpoint| {
            tokio::spawn(async move {
                endpoint.get_request().await;
                if endpoint.error().await.is_none() {
                    let status = endpoint.status().await;
                    let response_time = endpoint.response_time().await;
                    println!("{:?}", status);
                    println!("{:?}", response_time);

                    let influxdb_point = influxdb::generate_point(&endpoint).await;
                    println!("{:?}", influxdb_point);
                } else {
                    println!("{:?}", endpoint.error().await.unwrap());
                }
            });
        });
    }
}
