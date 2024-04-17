use std::fs;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use decryption::Candidate;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

mod transposition;
mod substitution;
mod key_generation;
mod scoring;
mod alphabet_analysis;
mod decryption;

use crate::key_generation::{generate_alphabets, decimate_alphabet, generate_keywords};
use crate::alphabet_analysis::{contains_keyword, score_alphabet};

fn char_to_number(c: char, alphabet: &str) -> Option<usize> {
    alphabet.find(c).map(|index| index + 1)
}

fn split_ciphertext(ciphertext: &str, alphabet: &str) -> (String, String) {
    let even_chars: String = ciphertext
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if let Some(num) = char_to_number(c, alphabet) {
                if num % 2 == 0 {
                    Some(c)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let odd_chars: String = ciphertext
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if let Some(num) = char_to_number(c, alphabet) {
                if num % 2 != 0 {
                    Some(c)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    (even_chars, odd_chars)
}

fn main() {
    // Read the K4 ciphertext from a file
    let ciphertext = fs::read_to_string("k4_ciphertext.txt").expect("Unable to read file");

    let top_n = 200; // Track top 10 results

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
    let alphabets = generate_alphabets(31, Some(31));
    let keywords = generate_keywords(1000000, 6);

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

    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();

    // Shared structure to collect results from all threads
    let top_candidates_global = Arc::new(Mutex::new(BinaryHeap::new()));

    pool.install(|| {
        alphabets.par_iter().enumerate().for_each(|(k, alphabet)| {
            let (even_ciphertext, odd_ciphertext) = split_ciphertext(&ciphertext, alphabet);

            let local_heap = Mutex::new(BinaryHeap::new());

            keywords.par_iter().for_each(|keyword| {
                transposition_techniques
                    .par_iter()
                    .enumerate()
                    .for_each(|(i, &transposition_fn)| {
                        substitution_techniques
                            .par_iter()
                            .enumerate()
                            .for_each(|(j, &substitution_fn)| {
                                let even_transposed = transposition_fn(&even_ciphertext, keyword);
                                let odd_transposed = transposition_fn(&odd_ciphertext, keyword);
                                let even_decimated_alphabet =
                                    decimate_alphabet(alphabet, keyword);
                                let odd_decimated_alphabet = decimate_alphabet(alphabet, keyword);
                                let even_plaintext =
                                    substitution_fn(&even_transposed, &even_decimated_alphabet);
                                let odd_plaintext =
                                    substitution_fn(&odd_transposed, &odd_decimated_alphabet);
                                let even_score = scoring::score_text(&even_plaintext);
                                let odd_score = scoring::score_text(&odd_plaintext);
                                let score = (even_score + odd_score) / 2.0;
                                let plaintext = even_plaintext + &odd_plaintext;
                                let candidate =
                                    Candidate::new(score, plaintext, i, j, k, keyword.clone());

                                let mut local_heap = local_heap.lock().unwrap();
                                local_heap.push(Reverse(candidate));
                                if local_heap.len() > top_n {
                                    local_heap.pop();
                                }
                            });
                    });
            });

            let local_heap = local_heap.into_inner().unwrap();
            let mut global_heap = top_candidates_global.lock().unwrap();
            for candidate in local_heap {
                global_heap.push(candidate);
                if global_heap.len() > top_n {
                    global_heap.pop();
                }
            }
        });
    });

    // Print results from global heap
    let top_candidates = top_candidates_global.lock().unwrap();
    println!("\nTop Candidates Summary:");
    for Reverse(candidate) in top_candidates.iter() {
        println!(
            "Score: {:.2}, Transposition: {}, Substitution: {}, Alphabet: {}, Keyword: {}, Plaintext: '{}'",
            candidate.score,
            candidate.transposition,
            candidate.substitution,
            candidate.alphabet_index,
            candidate.keyword,
            candidate.plaintext
        );
    }
}