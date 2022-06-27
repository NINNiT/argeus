use std::time::{Duration, Instant};

use reqwest::{Client, Error, Response, StatusCode};

pub struct MonitoringEndpoint {
    client: Client,
    url: String,
    error: Option<Error>,
    response: Option<Response>,
    response_time: Option<Duration>,
}

impl MonitoringEndpoint {
    pub fn new(client: Client, url: String) -> MonitoringEndpoint {
        return MonitoringEndpoint {
            client,
            url,
            error: None,
            response: None,
            response_time: None,
        };
    }

    pub async fn get_request(&mut self) {
        let watch = Instant::now();
        let response = self.client.get(&self.url).send().await;
        self.response_time = Some(watch.elapsed());

        match response {
            Ok(res) => {
                self.error = None;
                self.response = Some(res);
            }
            Err(err) => {
                self.error = Some(err);
                self.response = None;
            }
        }
    }

    pub async fn response_time(&self) -> Option<Duration> {
        return self.response_time;
    }

    pub async fn status(&self) -> Option<StatusCode> {
        if self.response.is_some() {
            return Some(self.response.as_ref().unwrap().status());
        } else {
            return None;
        }
    }

    pub async fn error(&self) -> Option<&Error> {
        return self.error.as_ref();
    }

    pub async fn url(&self) -> &str {
        return self.url.as_ref();
    }
}
