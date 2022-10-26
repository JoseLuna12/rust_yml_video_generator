use std::fmt;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct VideoContent {
    pub title: String,
    pub description: String,
    pub lang: String,
    pub font: String,
    pub bg: MediaAsset,
    pub music: Vec<MediaAsset>,
}

#[derive(Deserialize)]
pub struct MediaAsset {
    pub title: String,
    pub author: String,
    pub path: String,
}

impl fmt::Display for VideoContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Video: {}, videos: {}, music: {}",
            self.title,
            self.bg.title,
            self.music.len()
        )
    }
}

impl VideoContent {
    pub fn new(yaml_file: String) -> Option<VideoContent> {
        if let Ok(yaml_content) = std::fs::read_to_string(yaml_file) {
            let yaml_result: Result<VideoContent, serde_yaml::Error> =
                serde_yaml::from_str(&yaml_content);
            let result = match yaml_result {
                Ok(values) => Ok(values),
                Err(_) => Err(()),
            };
            if let Ok(video_content_result) = result {
                Some(video_content_result)
            } else {
                None
            }
        } else {
            None
        }
    }
}
