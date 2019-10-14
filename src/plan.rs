use std::fs;
use serde::Deserialize;
use crate::waveform;

#[derive(Deserialize)]
pub struct Speed {
    wpm: i32,
    farnsworth: i32,
}

#[derive(Deserialize)]
pub struct Lesson {
    pub title: String,
    pub words: Vec<String>,

    suppress_spoken: Option<bool>,
    base_speed: Speed,
    incr_speed: Speed,
}

pub fn get_lesson_plan() -> Vec<Lesson> {
    let file_content = fs::read_to_string("curriculum/config.yaml").unwrap();
    serde_yaml::from_str(&file_content).unwrap()
}

pub fn write_lesson_file(filename: &str, lesson: &Lesson) {
    let mut writer = hound::WavWriter::create(filename, waveform::get_wave_spec()).unwrap();

    let mut append_wav = |data: &[i32]| {
        for d in data {
            writer.write_sample(*d).unwrap();
        }
    };

    let word_boundary = waveform::gen_word_boundary(
        lesson.base_speed.wpm, lesson.base_speed.farnsworth);

    for word in &lesson.words {
        if lesson.suppress_spoken.unwrap_or(false) {
            let wave_file = format!("publish/words/{}.wav", &word);
            let spoken_wave = waveform::read_wav(&wave_file);
            append_wav(&spoken_wave);
            append_wav(&word_boundary);
        }

        let base_wave = waveform::gen_waveform(
            &word, lesson.base_speed.wpm, lesson.base_speed.farnsworth);
        let incr_wave = waveform::gen_waveform(
            &word, lesson.incr_speed.wpm, lesson.incr_speed.farnsworth);

        append_wav(&base_wave);
        append_wav(&word_boundary);
        append_wav(&base_wave);
        append_wav(&word_boundary);
        append_wav(&incr_wave);
        append_wav(&word_boundary);
        append_wav(&incr_wave);
        append_wav(&word_boundary);
        append_wav(&word_boundary);
    }
}
