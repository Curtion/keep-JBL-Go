#![windows_subsystem = "windows"]
use rodio::{source::Source, Decoder, OutputStream};
use std::io::Cursor;
use std::time::Duration;

fn main() {
    loop {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let buf = Cursor::new(include_bytes!("../slient.wav"));
        let source = Decoder::new(buf).expect("音频解码错误");
        stream_handle
            .play_raw(source.convert_samples())
            .expect("播放失败");
        std::thread::sleep(Duration::from_secs(60 * 5));
    }
}
