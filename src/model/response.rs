use serde::Deserialize;

use super::error::CobaltError;

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
        #[serde(skip_serializing_if = "Option::is_none")]
        is_hls: Option<bool>,
    },
    Picker {
        #[serde(skip_serializing_if = "Option::is_none")]
        audio: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        audio_filename: Option<String>,
        picker: Vec<PickerItem>,
    },
    Error {
        error: CobaltError,
    },
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
    pub album: Option<String>,
    pub copyright: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub track: Option<String>,
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
    pub kind: String, // "photo", "video", "gif"
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum LocalProcessingKind {
    Merge,
    Mute,
    Audio,
    Gif,
    Remux,
}
