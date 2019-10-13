use std::env;
use std::fs;
use std::collections::HashSet;
use std::io::Result;

fn get_word_list() -> Result<Vec<String>> {
    let file_content = fs::read_to_string("words.txt")?;
    Ok(file_content.lines().map(|x| x.to_string()).collect())
}

fn gen_incremental_list(base: &str, incr: &str) -> Result<()> {
    let incr_set: HashSet<_> = incr.chars().collect();
    let base_set: HashSet<_> = base.chars().collect();
    let total_set: HashSet<_> = incr_set.union(&base_set).cloned().collect();
    let words = get_word_list()?;

    let final_words: Vec<_> = words.iter().filter(|s| {
        let word_set: HashSet<_> = s.chars().collect();
        word_set.is_subset(&total_set) && incr_set.is_subset(&word_set)
    }).collect();

    println!("{:?}", &final_words);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    match args.as_slice() {
        [base, new_letters] => gen_incremental_list(&base, &new_letters),
        _ => panic!("Usage: gen_word_list <base letters> <incr letters>")
    }
}
