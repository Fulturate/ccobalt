use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct CobaltError {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl fmt::Display for CobaltError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code.to_ascii_lowercase().as_str() {
            "error.api.unreachable" => f.write_str("API unreachable (try again later)"),
            "error.api.timed_out" => f.write_str("API timeout (try again later)"),
            "error.api.rate_exceeded" => f.write_str("Rate limited (try again later)"),
            "error.api.capacity" => f.write_str("API busy (try again later)"),
            "error.api.generic" => f.write_str("General API error (try again later)"),
            "error.api.unknown_response" => f.write_str("Download failure. Make sure the link is valid. (unknown response)"),
            "error.api.service.unsupported" => f.write_str("That service or website is not supported."),
            "error.api.service.disabled" => f.write_str("Downloading from that service or website is temporarily disabled."),
            "error.api.link.invalid" => f.write_str("That link is invalid. Make sure it is correct."),
            "error.api.link.unsupported" => f.write_str("That link or format is unsupported."),
            "error.api.fetch.fail" => f.write_str("Failed to fetch the media. Make sure the link is valid, or try again later."),
            "error.api.fetch.critical" => f.write_str("Critical error fetching the media. Make sure the link is valid, or try again later."),
            "error.api.fetch.empty" => f.write_str("The service or website returned no data. This may be caused by the site blocking the downloader (try again later)"),
            "error.api.fetch.rate" => f.write_str("The service or website has rate limited the downloader (try again later)"),
            "error.api.fetch.short_link" => f.write_str("Unable to resolve the shortlink. Try using the full link to the media."),
            "error.api.content.too_long" => f.write_str("The requested content is too big."),
            "error.api.content.video.unavailable" => f.write_str("That video is unavailable. Make sure it is not region or age restricted, and is not private."),
            "error.api.content.video.live" => f.write_str("Live videos are unsupported."),
            "error.api.content.video.private" => f.write_str("That video is private."),
            "error.api.content.video.age" => f.write_str("That video is age restricted."),
            "error.api.content.video.region" => f.write_str("That video is region restricted."),
            "error.api.content.post.unavailable" => f.write_str("That post is unavailable. Make sure it is not region or age restricted, and is not private."),
            "error.api.content.post.private" => f.write_str("That post is private."),
            "error.api.content.post.age" => f.write_str("That post is age restricted."),
            "error.api.youtube.codec" => f.write_str("Missing YouTube codec. This is a bug."),
            "error.api.youtube.decipher" => f.write_str("Cannot decipher that video. Something probably broke."),
            "error.api.youtube.login" => f.write_str("That video requires a logged in account, which we do not have."),
            "error.api.youtube.token_expired" => f.write_str("Our YouTube token expired (try again later)"),
            "error.api.youtube.temporary_disabled" => f.write_str("YouTube support is temporarily disabled. Try again later."),
            _ => f.write_str(&self.code),
        }
    }
}

impl std::error::Error for CobaltError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = CobaltError {
            code: "error.api.unreachable".to_string(),
            context: None,
        };
        assert_eq!(format!("{}", error), "API unreachable (try again later)");
    }

    #[test]
    fn test_error_unknown() {
        let error = CobaltError {
            code: "error.api.unknown".to_string(),
            context: None,
        };
        assert_eq!(format!("{}", error), "error.api.unknown");
    }
}
