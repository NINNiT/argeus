use reqwest::{Client, Error, Response, StatusCode};

pub struct MonitoringEndpoint {
    client: Client,
    url: String,
    error: Option<Error>,
    last_response: Option<Response>,
}

impl MonitoringEndpoint {
    pub fn new(client: Client, url: String) -> MonitoringEndpoint {
        return MonitoringEndpoint {
            client,
            url,
            error: None,
            last_response: None,
        };
    }

    pub async fn get_request(&mut self) {
        let response = self.client.get(&self.url).send().await;

        match response {
            Ok(res) => {
                self.error = None;
                self.last_response = Some(res);
            }
            Err(err) => {
                self.error = Some(err);
                self.last_response = None;
            }
        }
    }

    pub async fn status(&self) -> Option<StatusCode> {
        if self.last_response.is_some() {
            return Some(self.last_response.as_ref().unwrap().status());
        } else {
            return None;
        }
    }

    pub async fn error(&self) -> Option<&Error> {
        return self.error.as_ref();
    }
}
