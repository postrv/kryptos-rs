use decryption::Candidate;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::sync::{Arc, Mutex};

mod alphabet_analysis;
mod decryption;
mod key_generation;
mod scoring;
mod substitution;

use crate::key_generation::generate_keywords;

fn create_char_to_number_lookup(alphabet: &str) -> HashMap<char, usize> {
    alphabet.chars().enumerate().map(|(i, c)| (c, i + 1)).collect()
}

type AlphabetKey = String;
type EvenOddCiphertexts = (String, String);

fn precompute_even_odd_ciphertexts(
    ciphertext: &str,
    alphabets: &[AlphabetKey],
    char_to_number_lookup: &HashMap<char, usize>,
) -> HashMap<AlphabetKey, EvenOddCiphertexts> {
    let mut even_odd_ciphertexts = HashMap::new();
    for alphabet in alphabets {
        let (even_chars, odd_chars) = split_ciphertext(ciphertext, char_to_number_lookup);
        even_odd_ciphertexts.insert(alphabet.clone(), (even_chars, odd_chars));
    }
    even_odd_ciphertexts
}

fn split_ciphertext(ciphertext: &str, char_to_number_lookup: &HashMap<char, usize>) -> (String, String) {
    let even_chars: String = ciphertext
        .chars()
        .enumerate()
        .filter_map(|(_i, c)| {
            if let Some(&num) = char_to_number_lookup.get(&c) {
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
        .filter_map(|(_i, c)| {
            if let Some(&num) = char_to_number_lookup.get(&c) {
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

    let substitution_techniques = vec![
        substitution::monoalphabetic_substitution,
        substitution::polyalphabetic_substitution,
        // Add more substitution techniques here
    ];

    // Define the 15th alphabet and its reverse
    let alphabet_15 = "KRYPTOSABCDEFGHIJLNGHIJLMNQUVWXZ";
    let alphabet_15_reverse = "ZXWVUQNMLJIHGNLJIHGFEDCBASOTPYRK";

    let base_alphabets = vec![alphabet_15.to_string(), alphabet_15_reverse.to_string()];

    let prioritized_keywords = vec![
        "BERLIN",
        "CLOCK",
        "EAST",
        "NORTHEAST",
        "SOUTHEAST",
        "WEST",
        "NORTHWEST",
        "SOUTHWEST",
        "FOLLOW",
        "LEFT",
        "HAND",
        "PATH",
        "RIGHT",
        "PARASYSTOLE",
        "LETHEAN",
        "NYPVTT",
        "ABSCICCA",
        "PALIMPSEST",
        "AGENTS",
        "BETWEEN",
        "SHADOW",
        "FORTY",
        "KRYPTOS",
        "NORTH",
        "SOUTH",
        "CLOCK",
        "HANDS",
        "KENNEDY",
        "BUSH",
        "ENTROPY"
        // Add more prioritized keywords
    ];

    // Generate a set of substitution keys
    let substitution_keys = generate_keywords(10000, 6);
    let substitution_keys_capacity = 10000;
    let mut substitution_keys_vec: Vec<String> = Vec::with_capacity(substitution_keys_capacity);
    substitution_keys_vec.extend(substitution_keys.clone());

    let mut unique_alphabets: Vec<String> = Vec::with_capacity(base_alphabets.len() * substitution_techniques.len() * substitution_keys.len());

    for base_alphabet in &base_alphabets {
        for &substitution_fn in &substitution_techniques {
            for substitution_key in &substitution_keys {
                let substituted_alphabet =
                    substitution_fn(base_alphabet, substitution_key, base_alphabet);
                unique_alphabets.push(substituted_alphabet);
            }
        }
    }

    let total_iterations =
        unique_alphabets.len() * substitution_keys.len() * substitution_techniques.len();
    let progress_interval = if total_iterations > 0 {
        total_iterations / 100
    } else {
        1
    }; // Print progress every 1% of total iterations


    let _iteration_count = 0;
    let progress_lock = Arc::new(Mutex::new(0));

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();

    // Shared structure to collect results from all threads
    let top_candidates_global = Arc::new(Mutex::new(BinaryHeap::new()));

    let char_to_number_lookup = create_char_to_number_lookup(alphabet_15);
    let even_odd_ciphertexts_map = precompute_even_odd_ciphertexts(&ciphertext, &base_alphabets, &char_to_number_lookup);

    pool.install(|| {
        unique_alphabets.par_iter().map(|s| s.as_str()).enumerate().for_each(|(k, alphabet)| {
            let (even_ciphertext, odd_ciphertext) = split_ciphertext(&ciphertext, &char_to_number_lookup);

            let local_heap = Mutex::new(BinaryHeap::new());

            prioritized_keywords.par_iter().for_each(|keyword| {
                substitution_techniques.par_iter().enumerate().for_each(|(j, &substitution_fn)| {
                    let even_plaintext = substitution_fn(&even_ciphertext, keyword, alphabet);
                    let odd_plaintext = substitution_fn(&odd_ciphertext, keyword, alphabet);
                    let plaintext = even_plaintext.clone() + &odd_plaintext;
                    let even_score = scoring::score_text(&even_plaintext);
                    let odd_score = scoring::score_text(&odd_plaintext);
                    let combined_score = scoring::score_text(&plaintext);
                    let score = (even_score + odd_score + combined_score) / 3.0;
                    let candidate = Candidate::new(
                        score,
                        plaintext,
                        0,
                        j,
                        k,
                        keyword.to_string(),
                        alphabet.to_string(),
                    );

                    let mut heap = local_heap.lock().unwrap_or_else(|poisoned| {
                        poisoned.into_inner()
                    });
                    heap.push(Reverse(candidate));
                    if heap.len() > top_n {
                        heap.pop();
                    }

                    // Update progress
                    let mut progress = progress_lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    *progress += 1;
                    if *progress % progress_interval == 0 {
                        let default_candidate = Candidate::new(0.0, String::new(), 0, 0, 0, String::new(), String::new()); // Provide the missing argument
                        let default_reverse = Reverse(default_candidate);
                        let current_best = heap.peek().unwrap_or(&default_reverse);
                        println!("Progress: {}%, Best Score: {:.8}, Keyword: {}, Plaintext: '{}'", *progress * 100 / total_iterations, current_best.0.score, current_best.0.keyword, current_best.0.plaintext);
                    }
                });
            });

            substitution_keys_vec.par_iter().for_each(|substitution_key| {
                substitution_techniques.par_iter().enumerate().for_each(|(j, &substitution_fn)| {
                    let even_plaintext = substitution_fn(&even_ciphertext, substitution_key, alphabet);
                    let odd_plaintext = substitution_fn(&odd_ciphertext, substitution_key, alphabet);
                    let plaintext = even_plaintext.clone() + &odd_plaintext;
                    let even_score = scoring::score_text(&even_plaintext);
                    let odd_score = scoring::score_text(&odd_plaintext);
                    let combined_score = scoring::score_text(&plaintext);
                    let score = (even_score + odd_score + combined_score) / 3.0;
                    let candidate = Candidate::new(
                        score,
                        plaintext,
                        0,
                        j,
                        k,
                        substitution_key.to_string(),
                        alphabet.to_string(),
                    );

                    let mut heap = local_heap.lock().unwrap_or_else(|poisoned| {
                        poisoned.into_inner()
                    });
                    heap.push(Reverse(candidate));
                    if heap.len() > top_n {
                        heap.pop();
                    }

                    // Update progress
                    let mut progress = progress_lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    *progress += 1;
                    if *progress % progress_interval == 0 {
                        let default_candidate = Candidate::new(0.0, String::new(), 0, 0, 0, String::new(), String::new()); // Provide the missing argument
                        let default_reverse = Reverse(default_candidate);
                        let current_best = heap.peek().unwrap_or(&default_reverse);
                        println!("Progress: {}%, Best Score: {:.8}, Keyword: {}, Plaintext: '{}'", *progress * 100 / total_iterations, current_best.0.score, current_best.0.keyword, current_best.0.plaintext);
                    }
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
            }
        );
    });


    // Print results from global heap
    let top_candidates = top_candidates_global.lock().unwrap();
    println!("\nTop Candidates Summary:");
    for Reverse(candidate) in top_candidates.iter() {
        println!(
            "Score: {:.8}, Substitution: {}, Alphabet: {}, Keyword: {}, Plaintext: '{}'",
            candidate.score,
            candidate.substitution,
            candidate.alphabet, // Print the alphabet
            candidate.keyword,
            candidate.plaintext
        );
    }
}
