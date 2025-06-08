use std::sync::Arc;

use crate::model::error::CobaltError;
use crate::model::request::DownloadRequest;
use crate::model::response::DownloadResponse;
use reqwest::{
    Client as HttpClient, Url,
    header::{AUTHORIZATION, CONTENT_TYPE},
};

#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    api_key: Option<String>,
    http: Arc<HttpClient>,
}

#[derive(Debug, Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn build(self) -> Result<Client, url::ParseError> {
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.cobalt.tools/".to_string());

        Ok(Client {
            base_url: base_url.parse()?,
            api_key: self.api_key,
            http: Arc::new(HttpClient::new()),
        })
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Sends a download request and returns the parsed response.
    pub async fn download(
        &self,
        request: &DownloadRequest,
    ) -> Result<DownloadResponse, CobaltError> {
        let mut req = self.http.post(self.base_url.clone()).json(request);

        req = req.header(AUTHORIZATION, "application/json");
        req = req.header(CONTENT_TYPE, "application/json");

        if let Some(key) = &self.api_key {
            req = req.header("Authorization", format!("Api-Key {key}"));
        }

        let res = req.send().await.map_err(|e| CobaltError {
            code: format!("error.api.unreachable: {}", e),
            context: None,
        })?;

        // let status = res.status();
        let body = res.text().await.map_err(|e| CobaltError {
            code: format!("error.api.timed_out: {}", e),
            context: None,
        })?;

        match serde_json::from_str::<DownloadResponse>(&body) {
            Ok(parsed) => Ok(parsed),
            Err(_) => Err(CobaltError {
                code: "error.api.unknown_response".into(),
                context: None,
            }),
        }
    }
}
