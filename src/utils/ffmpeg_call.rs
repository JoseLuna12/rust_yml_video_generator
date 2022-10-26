use std::process::Command;

use super::ffmpeg_commands::FfmpegComands;

pub enum FfmpegActions<'action_lt> {
    MergeMusic,
    LoopVideo(i16),
    CreateVideo(String),
    CreateThumbnail((&'action_lt str, &'action_lt str)),
}

fn ffmpeg_merge_final(input_path: String) {
    Command::new("ffmpeg")
        .args([
            "-stream_loop",
            "-1",
            "-i",
            &input_path,
            "-i",
            "output/output.mp3",
            "-shortest",
            "-map",
            "0:v:0",
            "-map",
            "1:a:0",
            "-y",
            "output/final.mp4",
        ])
        .output()
        .expect("failed to execute process");
}

fn ffmpeg_music(music_paths: &str) {
    println!("{}", format!("{}", music_paths));
    let output = Command::new("ffmpeg")
        .args([
            "-i",
            &format!("concat:{}", music_paths),
            "-c",
            "copy",
            "output/output.mp3",
        ])
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr))
}

fn loop_bg_video(video_path: &str, loop_time: i16) {
    Command::new("ffmpeg")
        .args([
            "-stream_loop",
            &loop_time.to_string(),
            "-i",
            video_path,
            "-c",
            "copy",
            "output/output.mp4",
        ])
        .output()
        .expect("failed to execute process");
}

fn thumbnail_generator(thumbnail_title: &str, font: &str) {
    let output_thumb1 = Command::new("ffmpeg")
        .args([
            "-ss",
            "00:00:15",
            "-i",
            "output/final.mp4",
            "-frames:v",
            "1",
            "output/frame.png",
        ])
        .output()
        .expect("failed to execute process");

    println!("status: {}", output_thumb1.status);
    println!("stdout: {}", String::from_utf8_lossy(&output_thumb1.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output_thumb1.stderr));

    let image_text = format!("drawtext=fontfile=fonts/{}:text='{}':fontcolor=white:fontsize=125:x=(w-text_w)/2:y=(h-text_h)/2", font,thumbnail_title);
    let output_thumb2 = Command::new("ffmpeg")
        .args([
            "-i",
            "output/frame.png",
            "-vf",
            &image_text,
            "-y",
            "output/thumbnail.jpg",
        ])
        .output()
        .expect("failed to execute process");

    println!("status: {}", output_thumb2.status);
    println!("stdout: {}", String::from_utf8_lossy(&output_thumb2.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output_thumb2.stderr));
}

pub fn call_ffmpeg(media: &str, action: FfmpegActions) {
    match action {
        FfmpegActions::MergeMusic => ffmpeg_music(media),
        FfmpegActions::LoopVideo(loop_time) => loop_bg_video(media, loop_time),
        FfmpegActions::CreateVideo(path) => ffmpeg_merge_final(path),
        FfmpegActions::CreateThumbnail((title, font)) => thumbnail_generator(title, font),
    };
}

pub fn call_ffmpeg_abstr(commands: FfmpegComands, loop_bg: bool) {
    let loop_bg_value = match loop_bg {
        true => FfmpegActions::LoopVideo(10),
        false => FfmpegActions::CreateVideo(commands.merge_file.0),
    };

    println!("... mergin songs");
    call_ffmpeg(&commands.concat_music, FfmpegActions::MergeMusic);
    println!("... creating video: {}", commands.background);
    call_ffmpeg(&commands.background, loop_bg_value);
    if loop_bg {
        call_ffmpeg(
            &commands.background,
            FfmpegActions::CreateVideo(commands.merge_file.1),
        );
    }
    call_ffmpeg(
        &commands.background,
        FfmpegActions::CreateThumbnail((
            &commands.current_video.title,
            &commands.current_video.font,
        )),
    )
}
