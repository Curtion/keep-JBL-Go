use rodio::buffer::SamplesBuffer;
use rodio::{source::Source, OutputStream};

fn main() {
    let st = SamplesBuffer::new(1, 44100, vec![1i16, 2, 3, 4, 5, 6]);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(st.convert_samples()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
}
