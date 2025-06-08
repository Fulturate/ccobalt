# CCobalt

CCobalt allows you to easily interact with the Cobalt API.

# Example

```rust
use ccobalt::{
    Client,
    model::request::{DownloadRequest, VideoQuality},
};

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .base_url("https://api.example.com/")
        .api_key("YOU_API_KEY")
        .build()
        .unwrap();

    let request = DownloadRequest {
        url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
        video_quality: Some(VideoQuality::Q4320),
        ..Default::default()
    };

    match client.download(&request).await {
        Ok(response) => {
            println!("Success: {:#?}", response);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
```
