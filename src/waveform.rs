use std::f64::consts::PI;
use hound;

const SAMPLE_RATE: u32 = 44100;
const SAMPLE_RATE_F: f64 = 44100.0;
const TONE: f64 = 440.0;

fn get_morse(c: char) -> &'static str {
    match c {
        'a' => ".-",
        'b' => "-...",
        'c' => "-.-.",
        'd' => "-..",
        'e' => ".",
        'f' => "..-.",
        'g' => "--.",
        'h' => "....",
        'i' => "..",
        'j' => ".---",
        'k' => "-.-",
        'l' => ".-..",
        'm' => "--",
        'n' => "-.",
        'o' => "---",
        'p' => ".--.",
        'q' => "--.-",
        'r' => ".-.",
        's' => "...",
        't' => "-",
        'u' => "..-",
        'v' => "...-",
        'w' => ".--",
        'x' => "-..-",
        'y' => "-.--",
        'z' => "--..",
        _ => panic!("Unknown char: {}", c)
    }
}

fn get_wave(length: usize) -> Vec<i32> {
    let max = (std::i32::MAX - 10) as f64;

    (0..length).map(|t| {
        let sample = (t as f64 / SAMPLE_RATE_F * TONE * 2.0 * PI).sin();
        (sample * max) as i32
    }).collect()
}

fn get_unit_times(wpm: i32, fwpm: i32) -> (usize, usize) {
    let wf = wpm as f64;
    let ff = fwpm as f64;

    let unit_time = 1.2 / wf;
    let f_unit_time = 60.0 / 19.0 / ff - 186.0 / 95.0 / wf;

    let unit = (unit_time * SAMPLE_RATE_F) as usize;
    let f_unit = (f_unit_time * SAMPLE_RATE_F) as usize;
    (unit, f_unit)
}

pub fn gen_word_boundary(wpm: i32, fwpm: i32) -> Vec<i32> {
    let (_, f_unit) = get_unit_times(wpm, fwpm);
    vec![0; f_unit * 7]
}

pub fn gen_waveform(content: &str, wpm: i32, fwpm: i32) -> Vec<i32> {
    assert!(fwpm <= wpm);
    let lower = content.to_lowercase();

    let mut result = Vec::new();

    let (unit, f_unit) = get_unit_times(wpm, fwpm);
    let dit = get_wave(unit);
    let dah = get_wave(unit * 3);
    let zeros = vec![0; unit];
    let f_zeros = vec![0; f_unit];

    let words: Vec<&str> = lower.split_whitespace().collect();

    for (wi, word) in words.iter().enumerate() {
        for (ci, c) in word.chars().enumerate() {
            let pattern = get_morse(c);

            for d in pattern.chars() {
                result.extend(if d == '.' { &dit } else { &dah });
                result.extend(&zeros);
            }

            if ci + 1 < word.len() {
                for _i in 0..3 {
                    result.extend(&f_zeros);
                }
            }
        }
        if wi + 1 < words.len() {
            for _i in 0..7 {
                result.extend(&f_zeros);
            }
        }
    }

    result
}

pub fn get_wave_spec() -> hound::WavSpec {
    hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int
    }
}

pub fn write_wav(filename: &str, wave_data: &[i32]) {
    let mut writer = hound::WavWriter::create(filename, get_wave_spec()).unwrap();
    for d in wave_data {
        writer.write_sample(*d).unwrap();
    }
}

pub fn read_wav(filename: &str) -> Vec<i32> {
    let reader = hound::WavReader::open(filename).unwrap();
    assert!(reader.spec() == get_wave_spec());
    reader.into_samples::<i32>().map(|s| s.unwrap()).collect()
}

