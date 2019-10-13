use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Speed {
    wpm: i32,
    farnsworth: i32,
}

#[derive(Deserialize)]
pub struct Lesson {
    pub title: String,
    pub words: Vec<String>,

    base_speed: Speed,
    incr_speed: Speed,
}

pub fn get_lesson_plan() -> Vec<Lesson> {
    let file_content = fs::read_to_string("curriculum/config.yaml").unwrap();
    serde_yaml::from_str(&file_content).unwrap()
}

pub fn write_lesson_file() {
}
