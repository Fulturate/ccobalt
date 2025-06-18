use super::error::CobaltError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InfoResponse {
    pub cobalt: CobaltInfo,
    pub git: GitInfo,
}

#[derive(Debug, Deserialize)]
pub struct CobaltInfo {
    pub version: String,
    pub url: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "turnstileSitekey")]
    pub turnstile_sitekey: String,
    pub services: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitInfo {
    pub branch: String,
    pub commit: String,
    pub remote: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum DownloadResponse {
    Tunnel {
        url: String,
        filename: String,
    },
    Redirect {
        url: String,
        filename: String,
    },
    LocalProcessing {
        #[serde(rename = "type")]
        kind: LocalProcessingKind,
        service: String,
        tunnel: Vec<String>,
        output: Output,
        #[serde(skip_serializing_if = "Option::is_none")]
        audio: Option<Audio>,
        #[serde(rename = "isHLS", skip_serializing_if = "Option::is_none")]
        is_hls: Option<bool>,
    },
    Picker {
        picker: Vec<PickerItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        audio: Option<String>,
        #[serde(rename = "audioFilename", skip_serializing_if = "Option::is_none")]
        audio_filename: Option<String>,
    },
    Error {
        error: CobaltError,
    },
}

impl DownloadResponse {
    pub fn is_error(&self) -> bool {
        matches!(self, DownloadResponse::Error { .. })
    }

    pub fn is_tunnel(&self) -> bool {
        matches!(self, DownloadResponse::Tunnel { .. })
    }

    pub fn is_redirect(&self) -> bool {
        matches!(self, DownloadResponse::Redirect { .. })
    }

    pub fn is_local_processing(&self) -> bool {
        matches!(self, DownloadResponse::LocalProcessing { .. })
    }

    pub fn is_picker(&self) -> bool {
        matches!(self, DownloadResponse::Picker { .. })
    }

    /// Get the download URL if available.
    pub fn get_download_url(&self) -> Option<String> {
        match self {
            DownloadResponse::Tunnel { url, .. } => Some(url.clone()),
            DownloadResponse::Redirect { url, .. } => Some(url.clone()),
            DownloadResponse::LocalProcessing { tunnel, .. } => {
                Some(tunnel.first().cloned().unwrap_or_default())
            }
            DownloadResponse::Picker { .. } => None,
            DownloadResponse::Error { .. } => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Output {
    #[serde(rename = "type")]
    pub mime_type: String,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OutputMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct OutputMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Audio {
    pub copy: bool,
    pub format: String,
    pub bitrate: String,
}

#[derive(Debug, Deserialize)]
pub struct PickerItem {
    #[serde(rename = "type")]
    pub kind: String, // Expected to be "photo", "video", or "gif"
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocalProcessingKind {
    Merge,
    Mute,
    Audio,
    Gif,
    Remux,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_tunnel_response() {
        let json = r#"{
            "status": "tunnel",
            "url": "https://example.com/video.mp4",
            "filename": "video.mp4"
        }"#;

        let res: DownloadResponse = from_str(json).unwrap();
        if let DownloadResponse::Tunnel { url, filename } = res {
            assert_eq!(filename, "video.mp4");
            assert!(url.contains("example.com"));
        } else {
            panic!("Expected tunnel response");
        }
    }
}
