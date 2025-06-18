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
        .api_key("YOUR_API_KEY")
        .build()
        .expect("Failed to build client");

    let request = DownloadRequest {
        url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
        video_quality: Some(VideoQuality::Q4320),
        ..Default::default()
    };

    match client.download_and_save(&request, "download", ".").await {
        Ok(path) => {
            println!("File saved to: {:?}", path);
        }
        Err(err) => {
            eprintln!("Download failed: {}", err);
        }
    }
}
```
