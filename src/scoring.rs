
// scoring.rs

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref LETTER_FREQUENCIES: HashMap<char, f64> = {
        let mut freq_map = HashMap::new();
        freq_map.insert('A', 0.0817);
        freq_map.insert('B', 0.0150);
        freq_map.insert('C', 0.0278);
        freq_map.insert('D', 0.0425);
        freq_map.insert('E', 0.1270);
        freq_map.insert('F', 0.0223);
        freq_map.insert('G', 0.0202);
        freq_map.insert('H', 0.0609);
        freq_map.insert('I', 0.0697);
        freq_map.insert('J', 0.0015);
        freq_map.insert('K', 0.0077);
        freq_map.insert('L', 0.0403);
        freq_map.insert('M', 0.0241);
        freq_map.insert('N', 0.0675);
        freq_map.insert('O', 0.0751);
        freq_map.insert('P', 0.0193);
        freq_map.insert('Q', 0.0010);
        freq_map.insert('R', 0.0599);
        freq_map.insert('S', 0.0633);
        freq_map.insert('T', 0.0906);
        freq_map.insert('U', 0.0276);
        freq_map.insert('V', 0.0098);
        freq_map.insert('W', 0.0236);
        freq_map.insert('X', 0.0015);
        freq_map.insert('Y', 0.0197);
        freq_map.insert('Z', 0.0007);
        freq_map
    };
}

pub fn score_text(text: &str) -> f64 {
    let mut score = 0.0;
    let text_length = text.len() as f64;

    for c in text.chars() {
        if let Some(&freq) = LETTER_FREQUENCIES.get(&c.to_ascii_uppercase()) {
            score += freq;
        }
    }

    score / text_length
}