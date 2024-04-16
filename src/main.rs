// main.rs

use std::fs;

mod transposition;
mod substitution;
mod key_generation;
mod scoring;
mod alphabet_analysis;

use crate::key_generation::{generate_keywords, generate_alphabets, decimate_alphabet};

use crate::alphabet_analysis::{contains_keyword, score_alphabet};

fn main() {
    // Read the K4 ciphertext from a file
    let ciphertext = fs::read_to_string("k4_ciphertext.txt").expect("Unable to read file");

    // Define a range of transposition and substitution techniques to try
    let transposition_techniques = vec![
        transposition::columnar_transposition,
        transposition::route_transposition,
        // Add more transposition techniques here
    ];

    let substitution_techniques = vec![
        substitution::monoalphabetic_substitution,
        substitution::polyalphabetic_substitution,
        // Add more substitution techniques here
    ];

    // Generate a range of alphabets and keywords to test
    let alphabets = generate_alphabets(31);
    let keywords = generate_keywords(11, 8);

    // Analyze the generated alphabets
    for (k, alphabet) in alphabets.iter().enumerate() {
        let alphabet_score = score_alphabet(alphabet);
        println!("Alphabet: {}, Score: {}", k, alphabet_score);

        for keyword in &keywords {
            if contains_keyword(alphabet, keyword) {
                println!("Alphabet {} contains keyword: {}", k, keyword);
            }
        }
    }

    // Iterate through all combinations of techniques, alphabets, and keywords
    for (i, &transposition_fn) in transposition_techniques.iter().enumerate() {
        for (j, &substitution_fn) in substitution_techniques.iter().enumerate() {
            for (k, alphabet) in alphabets.iter().enumerate() {
                for (l, keyword) in keywords.iter().enumerate() {
                    let transposed_text = transposition_fn(&ciphertext, keyword);
                    let decimated_alphabet = decimate_alphabet(alphabet, keyword);
                    let plaintext_candidate = substitution_fn(&transposed_text, &decimated_alphabet);
                    let score = scoring::score_text(&plaintext_candidate);
                    println!("Transposition: {}, Substitution: {}, Alphabet: {}, Keyword: {}, Score: {}, Plaintext: {}",
                             i, j, k, l, score, plaintext_candidate);
                }
            }
        }
    }
}