// substitution.rs

use std::collections::HashMap;

pub fn monoalphabetic_substitution(text: &str, key: &str, alphabet: &str) -> String {
    let mut key_map = HashMap::new();
    let mut key_index = 0;

    // Create a mapping from the alphabet to the keyword
    for char in alphabet.chars() {
        if key_index < key.len() {
            key_map.insert(char, key.chars().nth(key_index).unwrap());
            key_index += 1;
        } else {
            // If the keyword is exhausted, use the original character
            key_map.insert(char, char);
        }
    }

    // Apply the mapping to the text
    text.chars()
        .map(|c| *key_map.get(&c).unwrap_or(&c))
        .collect()
}

pub fn polyalphabetic_substitution(text: &str, key: &str, alphabet: &str) -> String {
    let mut key_iter = key.chars().cycle();
    text.chars()
        .map(|c| {
            if let Some(pos) = alphabet.find(c) {
                let shift = alphabet.find(key_iter.next().unwrap_or('A')).unwrap_or(0);
                alphabet
                    .chars()
                    .nth((pos + shift) % alphabet.len())
                    .unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}
