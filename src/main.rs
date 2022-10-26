mod utils;

fn main() {
    let yaml_location = std::env::args().nth(1).expect("no Yaml file given");
    let test = utils::yaml_reader::VideoContent::new(yaml_location);
    match test {
        Some(value) => {
            let exc_commands = utils::ffmpeg_commands::FfmpegComands::new(value);
            utils::ffmpeg_call::call_ffmpeg_abstr(exc_commands, false);
        }
        None => println!("Problem reading"),
    }
}
