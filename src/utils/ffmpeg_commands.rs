use std::fmt;

use super::yaml_reader::{MediaAsset, VideoContent};

pub struct FfmpegComands {
    pub concat_music: String,
    pub background: String,
    pub merge_file: (String, String),
    pub current_video: VideoContent,
}

fn get_concat_audio(media_asset: &Vec<MediaAsset>) -> String {
    let mut media_ffmpeg = String::from("");
    for song in media_asset {
        media_ffmpeg.push_str(&format!("{}|", song.path));
    }

    let music_list = &media_ffmpeg[0..media_ffmpeg.len() - 1];
    format!("{music_list}")
}

fn get_loop_video(background: &MediaAsset) -> String {
    background.path.to_owned()
}

impl FfmpegComands {
    pub fn new(content: VideoContent) -> Self {
        FfmpegComands {
            concat_music: get_concat_audio(&content.music),
            background: get_loop_video(&content.bg),
            merge_file: (content.bg.path.to_owned(), "/output/output.mp4".to_owned()),
            current_video: content,
            // current_video_title: content.title,
        }
    }
}

impl fmt::Display for FfmpegComands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "bg: {}, Songs: {}",
            self.background,
            self.concat_music.len()
        )
    }
}
