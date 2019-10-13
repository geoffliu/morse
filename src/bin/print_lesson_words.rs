use morse::plan;
use std::collections::HashSet;

fn main() {
    let lessons = plan::get_lesson_plan();
    let mut all_words: HashSet<String> = HashSet::new();

    for lesson in lessons {
        all_words.extend(lesson.words.iter().cloned());
    }

    for word in all_words {
        println!("{}", &word);
    }
}
