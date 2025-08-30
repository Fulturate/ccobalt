use crate::model::request::DownloadRequest;
use crate::model::response::DownloadResponse;
use crate::model::{error::CobaltError, response::InfoResponse};
use log::info;
use reqwest::{
    Client as HttpClient, Url,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    api_key: Option<String>,
    bearer_token: Option<String>,
    http: Arc<HttpClient>,
}

#[derive(Debug, Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    bearer_token: Option<String>,
    http: Option<Arc<HttpClient>>,
    user_agent: Option<String>,
}

impl ClientBuilder {
    /// Creates a new `ClientBuilder` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL for the API.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());

        if !self.base_url.as_ref().unwrap().ends_with('/') {
            self.base_url = Some(format!("{}/", self.base_url.as_ref().unwrap()));
        }

        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Sets the API key for authentication.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Sets the bearer token for authentication.
    pub fn bearer_token(mut self, token: impl Into<String>) -> Self {
        self.bearer_token = Some(token.into());
        self
    }

    /// Sets the HTTP client to use for requests.
    ///
    /// If not set, a default `reqwest::Client` will be used.
    pub fn http_client(mut self, client: HttpClient) -> Self {
        self.http = Some(Arc::new(client));
        self
    }

    /// Builds the `Client` instance.
    pub fn build(self) -> Result<Client, url::ParseError> {
        let base_url = self.base_url.expect("base_url is required");

        if self.api_key.is_none() && self.bearer_token.is_none() {
            panic!("Must set either api_key or bearer_token");
        }

        if self.api_key.is_some() && self.bearer_token.is_some() {
            panic!("Cannot set both api_key and bearer_token");
        }

        let http_client = self.http.unwrap_or_else(|| {
            Arc::new(
                HttpClient::builder()
                    .user_agent(
                        self.user_agent
                            .unwrap_or_else(|| "ccobalt/0.0.1 (+client)".to_string()),
                    )
                    .build()
                    .unwrap(),
            )
        });

        Ok(Client {
            base_url: base_url.parse()?,
            api_key: self.api_key,
            bearer_token: self.bearer_token,
            http: http_client,
        })
    }
}

impl Client {
    /// Creates a new `ClientBuilder` to configure and build a `Client`.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Retrieves information about the API, such as version and supported features.
    pub async fn get_info(&self) -> Result<InfoResponse, CobaltError> {
        let mut req = self.http.get(self.base_url.clone());

        req = req.header(ACCEPT, "application/json");

        let res = req.send().await.map_err(|_| CobaltError {
            code: "error.api.unreachable".into(),
            context: None,
        })?;

        info!("{:#?}", res.headers());

        // let status = res.status();
        let body = res.text().await.map_err(|_| CobaltError {
            code: "error.api.timed_out".into(),
            context: None,
        })?;

        match serde_json::from_str::<InfoResponse>(&body) {
            Ok(parsed) => Ok(parsed),
            Err(_) => Err(CobaltError {
                code: "error.api.unknown_response".into(),
                context: None,
            }),
        }
    }

    /// Resolves a download request and returns the download response.
    pub async fn resolve_download(
        &self,
        request: &DownloadRequest,
    ) -> Result<DownloadResponse, CobaltError> {
        let mut req = self.http.post(self.base_url.clone()).json(request);

        req = req.header(ACCEPT, "application/json");
        req = req.header(CONTENT_TYPE, "application/json");

        if let Some(key) = &self.api_key {
            req = req.header(AUTHORIZATION, format!("Api-Key {key}"));
        } else if let Some(token) = &self.bearer_token {
            req = req.header(AUTHORIZATION, format!("Bearer {token}"));
        }

        let res = req.send().await.map_err(|_| CobaltError {
            code: "error.api.unreachable".into(),
            context: None,
        })?;

        // let status = res.status();
        let body = res.text().await.map_err(|_| CobaltError {
            code: "error.api.timed_out".into(),
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

    /// Retrieves download information and downloads the file from the stream URL if available.
    pub async fn download(&self, request: &DownloadRequest) -> Result<Vec<u8>, CobaltError> {
        let response = self.resolve_download(request).await?;

        if let Some(url) = response.get_download_url() {
            let url = Url::from_str(&url).map_err(|_| CobaltError {
                code: "error.api.invalid_url".into(),
                context: None,
            })?;

            let stream = crate::util::stream::read_stream(Arc::clone(&self.http), url)
                .await
                .map_err(|_| CobaltError {
                    code: "error.api.download_failed".into(),
                    context: None,
                })?;
            Ok(stream)
        } else {
            Err(CobaltError {
                code: "error.api.no_download_url".into(),
                context: None,
            })
        }
    }

    /// Download and save the file to the specified directory.
    pub async fn download_and_save(
        &self,
        request: &DownloadRequest,
        base_name: &str,
        directory: &str,
    ) -> Result<std::path::PathBuf, CobaltError> {
        let bytes = self.download(request).await?;
        crate::util::write::save_to_file(&bytes, base_name, directory).map_err(|_| CobaltError {
            code: "error.api.save_failed".into(),
            context: None,
        })
    }
}
