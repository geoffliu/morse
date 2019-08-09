use std::env;
use morse::waveform;
use hound;

fn write_file(filename: &str) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: waveform::SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for d in waveform::gen_waveform("abc abc abc", 10, 8) {
        writer.write_sample(d).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Option::Some(s) => write_file(s.as_str()),
        _ => panic!("No file name")
    }
}
