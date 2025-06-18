use std::sync::Arc;

use futures::StreamExt;
use reqwest::Client;
use url::Url;

/// Reads a stream from the given URL and returns the full response body as bytes.
pub async fn read_stream(client: Arc<Client>, url: Url) -> Result<Vec<u8>, reqwest::Error> {
    let response = client.get(url).send().await?;

    let mut stream = response.bytes_stream();
    let mut data = Vec::new();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        data.extend_from_slice(&bytes);
    }

    Ok(data)
}
