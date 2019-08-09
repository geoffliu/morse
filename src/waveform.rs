use std::f64::consts::PI;

const SAMPLE_RATE: f64 = 44100.0;
const TONE: f64 = 440.0;

fn get_morse(c: char) -> &'static str {
    match c {
        'a' => ".-",
        'b' => "-...",
        'c' => "-.-.",
        _ => panic!("Unknown char")
    }
}

fn get_wave(length: usize) -> Vec<i32> {
    let max = (std::i32::MAX - 10) as f64;

    (0..length).map(|t| {
        let sample = (t as f64 / SAMPLE_RATE * TONE * 2.0 * PI).sin();
        (sample * max) as i32
    }).collect()
}

pub fn gen_waveform(content: &str, wpm: i32, fwpm: i32) -> Vec<i32> {
    let mut result = Vec::new();

    let wf = wpm as f64;
    let ff = fwpm as f64;

    let unit_time = 1.2 / wf;
    let f_unit_time = 60.0 / 19.0 / ff - 186.0 / 95.0 / wf;

    let unit = (unit_time * SAMPLE_RATE) as usize;
    let f_unit = (f_unit_time * SAMPLE_RATE) as usize;

    let dit = get_wave(unit);
    let dah = get_wave(unit * 3);
    let zeros = vec![0; unit];
    let f_zeros = vec![0; f_unit];

    for word in content.split_whitespace() {
        for c in word.chars() {
            let pattern = get_morse(c);

            for d in pattern.chars() {
                result.extend(if d == '.' { &dit } else { &dah });
                result.extend(&zeros);
            }

            for _i in 0..3 {
                result.extend(&f_zeros);
            }
        }
        for _i in 0..7 {
            result.extend(&f_zeros);
        }
    }

    result
}
