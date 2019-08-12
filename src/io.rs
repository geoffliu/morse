use crate::waveform;
use hound;

pub fn write_wav(filename: &str, wave_data: &[i32]) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: waveform::SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for d in wave_data {
        writer.write_sample(*d).unwrap();
    }
}

