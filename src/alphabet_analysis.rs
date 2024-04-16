// alphabet_analysis.rs

use std::collections::HashSet;

pub fn contains_keyword(alphabet: &str, keyword: &str) -> bool {
    alphabet.contains(keyword)
}

pub fn score_substring(substring: &str) -> f64 {
    // Define a set of common English words
    let common_words: HashSet<&str> = vec![
        "THE", "AND", "THAT", "HAVE", "FOR", "NOT", "WITH", "YOU", "THIS", "BUT", "KRYPTOS", "ABSCICCA", "PALIMPSEST", "AGENTS", "BETWEEN", "CLOCK", "SHADOW", "FORTY", "IT", "IS", "BERLIN", "NORTH", "EAST", "SOUTH", "WEST", "FOLLOW", "LEFT", "HAND", "PATH", "NORTHEAST", "SOUTHEAST", "SOUTHWEST", "NORTHWEST", "RIGHT", "PARASYSTOLE", "LETHEAN", "AN"
        // Add more common words as needed
    ].into_iter().collect();

    let uppercase_substring = substring.to_uppercase();

    if common_words.contains(uppercase_substring.as_str()) {
        1.0
    } else {
        0.0
    }
}

pub fn score_alphabet(alphabet: &str) -> f64 {
    let min_substring_length = 3;
    let max_substring_length = 6;
    let mut total_score = 0.0;
    let mut substring_count = 0;

    for length in min_substring_length..=max_substring_length {
        for i in 0..=(alphabet.len() - length) {
            let substring = &alphabet[i..i + length];
            let substring_score = score_substring(substring);
            total_score += substring_score;
            substring_count += 1;
        }
    }

    if substring_count > 0 {
        total_score / substring_count as f64
    } else {
        0.0
    }
}