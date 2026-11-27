use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, Method, Response};

use crate::net::egress::Proxy;
use crate::support::error::{AppError, AppResult};

const TIMEOUT_SECONDS: u64 = 90;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(proxy: Option<&Proxy>) -> AppResult<HttpClient> {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/plain, */*"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(crate::support::consts::app::AGENT),
        );

        let mut builder = Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECONDS))
            .default_headers(headers)
            .connect_timeout(Duration::from_secs(20));

        if let Some(route) = proxy {
            let url = reqwest::Proxy::all(route.url()).map_err(|err| {
                AppError::network(format!("the proxy could not be built: {}", err))
            })?;
            builder = builder.proxy(url);
        }

        let client = builder
            .build()
            .map_err(|err| AppError::network(format!("the client could not be built: {}", err)))?;
        Ok(HttpClient { client })
    }

    pub async fn send(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
        extra: &[(&str, &str)],
    ) -> AppResult<Response> {
        let mut request = self.client.request(method, url);
        for (name, value) in extra {
            request = request.header(*name, *value);
        }
        if let Some(text) = body {
            request = request.body(text);
        }
        request
            .send()
            .await
            .map_err(|err| AppError::network(format!("the request did not complete: {}", err)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_client_builds_without_a_proxy() {
        assert!(HttpClient::new(None).is_ok());
    }

    #[test]
    fn a_client_builds_with_a_proxy() {
        let proxy = Proxy::parse("http://203.0.113.1:8080").unwrap();
        assert!(HttpClient::new(Some(&proxy)).is_ok());
    }
}
