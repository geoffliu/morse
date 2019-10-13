use morse::plan;

fn main() {
    let lessons = plan::get_lesson_plan();
    for (i, lesson) in lessons.iter().enumerate() {
        let filename = format!("publish/lesson{:03}.wav", i + 1);
        plan::write_lesson_file(&filename, &lesson);
    }
}
