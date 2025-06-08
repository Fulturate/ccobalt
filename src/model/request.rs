use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRequest {
    pub url: String, // required

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_bitrate: Option<AudioBitrate>, // default: 128

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_format: Option<AudioFormat>, // default: mp3

    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_mode: Option<DownloadMode>, // default: auto

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename_style: Option<FilenameStyle>, // default: basic

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<VideoQuality>, // default: 1080

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_metadata: Option<bool>, // default: false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_proxy: Option<bool>, // default: false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_processing: Option<bool>, // default: false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_video_codec: Option<YoutubeVideoCodec>, // h264 / av1 / vp9

    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_dub_lang: Option<String>, // e.g. "en", "zh-CN"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_gif: Option<bool>, // default: true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_h265: Option<bool>, // default: false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok_full_audio: Option<bool>, // default: false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_better_audio: Option<bool>, // default: false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_hls: Option<bool>, // default: false
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioBitrate {
    #[serde(rename = "320")]
    Kbps320,
    #[serde(rename = "256")]
    Kbps256,
    #[serde(rename = "128")]
    Kbps128,
    #[serde(rename = "96")]
    Kbps96,
    #[serde(rename = "64")]
    Kbps64,
    #[serde(rename = "8")]
    Kbps8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Best,
    Mp3,
    Ogg,
    Wav,
    Opus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadMode {
    Auto,
    Audio,
    Mute,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilenameStyle {
    Classic,
    Pretty,
    Basic,
    Nerdy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoQuality {
    Max,
    #[serde(rename = "4320")]
    Q4320,
    #[serde(rename = "2160")]
    Q2160,
    #[serde(rename = "1440")]
    Q1440,
    #[serde(rename = "1080")]
    Q1080,
    #[serde(rename = "720")]
    Q720,
    #[serde(rename = "480")]
    Q480,
    #[serde(rename = "360")]
    Q360,
    #[serde(rename = "240")]
    Q240,
    #[serde(rename = "144")]
    Q144,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YoutubeVideoCodec {
    H264,
    Av1,
    Vp9,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_request_serialization() {
        let request = DownloadRequest {
            url: "https://example.com/video".to_string(),
            audio_bitrate: Some(AudioBitrate::Kbps128),
            audio_format: Some(AudioFormat::Mp3),
            download_mode: Some(DownloadMode::Auto),
            filename_style: Some(FilenameStyle::Pretty),
            video_quality: Some(VideoQuality::Q1080),
            disable_metadata: Some(false),
            always_proxy: Some(false),
            local_processing: Some(true),
            youtube_video_codec: Some(YoutubeVideoCodec::H264),
            youtube_dub_lang: Some("en".to_string()),
            convert_gif: Some(true),
            allow_h265: Some(false),
            tiktok_full_audio: Some(false),
            youtube_better_audio: Some(true),
            youtube_hls: Some(false),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        println!("{}", serialized);
    }
}
