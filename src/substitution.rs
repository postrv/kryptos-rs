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
    let key_len = key.len();
    let alphabet_len = alphabet.len();

    text.chars()
        .enumerate()
        .map(|(i, c)| {
            if let Some(pos) = alphabet.find(c) {
                let key_char = key.chars().nth(i % key_len).unwrap_or('A');
                let shift = alphabet.find(key_char).unwrap_or(0);
                alphabet
                    .chars()
                    .nth((pos + shift) % alphabet_len)
                    .unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}

pub fn generate_shifted_alphabets(key: &str, alphabet: &str) -> Vec<String> {
    let key_len = key.len();
    let alphabet_len = alphabet.len();

    (0..alphabet_len)
        .map(|shift| {
            alphabet
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let key_char = key.chars().nth((i + shift) % key_len).unwrap_or('A');
                    let pos = alphabet.find(key_char).unwrap_or(0);
                    alphabet.chars().nth((i + pos) % alphabet_len).unwrap_or(c)
                })
                .collect()
        })
        .collect()
}
