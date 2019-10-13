use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Speed {
    pub wpm: i32,
    pub farnsworth: i32,
}

#[derive(Deserialize)]
pub struct Lesson {
    pub title: String,
    pub base_speed: Speed,
    pub incr_speed: Speed,
    pub words: Vec<String>,
}

pub fn get_lesson_plan() -> Vec<Lesson> {
    let file_content = fs::read_to_string("lessons.yaml").unwrap();
    serde_yaml::from_str(&file_content).unwrap()
}


