// substitution.rs

use std::collections::HashMap;

pub fn monoalphabetic_substitution(text: &str, key: &str) -> String {
    let mut substitution_map = HashMap::new();
    let alphabet = "NGHIJLMNQUVWXZKRYPTOSABCDEFGHIJL";

    for (i, c) in key.chars().enumerate() {
        substitution_map.insert(alphabet.chars().nth(i).unwrap(), c);
    }

    text.chars()
        .map(|c| {
            if let Some(&sub_char) = substitution_map.get(&c.to_ascii_uppercase()) {
                sub_char
            } else {
                c
            }
        })
        .collect()
}


pub fn polyalphabetic_substitution(text: &str, key: &str) -> String {
    let mut key_iter = key.chars().cycle();
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let shift = key_iter.next().unwrap() as u8 - b'A';
                ((c as u8 - b'A' + shift) % 26 + b'A') as char
            } else {
                c
            }
        })
        .collect()
}
