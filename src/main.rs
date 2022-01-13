#![windows_subsystem = "windows"]
use rodio::buffer::SamplesBuffer;
use rodio::{source::Source, OutputStream};

fn main() {
    loop {
        let st = SamplesBuffer::new(1, 44100, vec![1i16, 1, 1, 1, 1, 1]);
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        stream_handle.play_raw(st.convert_samples()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(60 * 5));
    }
}
