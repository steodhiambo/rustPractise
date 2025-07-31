use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for raw in words.split_whitespace() {
        // Keep only alphanumerics and apostrophes, then trim outer apostrophes/punctuation
        let cleaned = raw
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .collect::<String>();

        // Now trim apostrophes or punctuation from start and end
        let word = cleaned
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();

        if !word.is_empty() {
            *map.entry(word).or_insert(0) += 1;
        }
    }

    map
}