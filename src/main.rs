mod utils;

fn main() {
    let test = utils::yaml_reader::VideoContent::new(String::from("./src/test.yaml"));
    match test {
        Some(value) => {
            let exc_commands = utils::ffmpeg_commands::FfmpegComands::new(value);
            utils::ffmpeg_call::call_ffmpeg_abstr(exc_commands, false);
        }
        None => println!("Problem reading"),
    }
}
