use decryption::Candidate;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::sync::{Arc, Mutex};

mod alphabet_analysis;
mod decryption;
mod key_generation;
mod scoring;
mod substitution;

use crate::key_generation::generate_keywords_from_wordlist;

fn main() {
    // Read the K4 ciphertext from a file
    let ciphertext = fs::read_to_string("k4_ciphertext.txt").expect("Unable to read file");

    let substitution_techniques = vec![
        substitution::polyalphabetic_substitution,
        // Add more substitution techniques here
    ];

    // Define the 15th alphabet and its reverse
    let alphabet_15 = "KRYPTOSABCDEFGHIJLNGHIJLMNQUVWXZ";

    let base_alphabets = vec![
        alphabet_15.to_string(),
    ];

    // Generate substitution keys from the wordlist
    let wordlist_file = "/usr/share/dict/words";
    let keyword_length = 11;
    let substitution_keys = generate_keywords_from_wordlist(wordlist_file, keyword_length);

    let mut poly_alphabets: Vec<String> = Vec::new();

    for base_alphabet in &base_alphabets {
        for &substitution_fn in &substitution_techniques {
            for substitution_key in &substitution_keys {
                let substituted_alphabet =
                    substitution_fn(base_alphabet, substitution_key, base_alphabet);
                poly_alphabets.push(substituted_alphabet);
            }
        }
    }

    let total_iterations = poly_alphabets.len() * substitution_keys.len();
    let progress_interval = if total_iterations > 0 {
        total_iterations / 100
    } else {
        1
    }; // Print progress every 1% of total iterations

    let progress_lock = Arc::new(Mutex::new(0));

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();

    // Shared structure to collect results from all threads
    let top_poly_candidates = Arc::new(Mutex::new(BinaryHeap::new()));

    pool.install(|| {

        poly_alphabets
            .par_iter()
            .map(|s| s.as_str())
            .enumerate()
            .for_each(|(k, alphabet)| {
                let local_poly_heap = Mutex::new(BinaryHeap::new());

                substitution_keys.par_iter().for_each(|substitution_key| {
                    let plaintext = substitution::polyalphabetic_substitution(
                        &ciphertext,
                        substitution_key,
                        alphabet,
                    );
                    let score = scoring::score_text(&plaintext);
                    let candidate = Candidate::new(
                        score,
                        plaintext,
                        0,
                        1,
                        k,
                        substitution_key.to_string(),
                        alphabet.to_string(),
                    );

                    let mut heap = local_poly_heap
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner());
                    heap.push(Reverse(candidate));
                    if heap.len() > 250 {
                        heap.pop();
                    }
                    // Update progress
                    let mut progress = progress_lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    *progress += 1;
                    if *progress % progress_interval == 0 {
                        let default_candidate = Candidate::new(0.0, String::new(), 0, 0, 0, String::new(), String::new());
                        let default_reverse = Reverse(default_candidate);
                        let current_best = heap.peek().unwrap_or(&default_reverse);
                        println!("Progress: {}%, Best Score: {:.8}, Keyword: {}, Plaintext: '{}'", *progress * 100 / total_iterations, current_best.0.score, current_best.0.keyword, current_best.0.plaintext);
                    }
                });

                let local_poly_heap = local_poly_heap.into_inner().unwrap();
                let mut global_poly_heap = top_poly_candidates.lock().unwrap();
                for candidate in local_poly_heap {
                    global_poly_heap.push(candidate);
                    if global_poly_heap.len() > 250 {
                        global_poly_heap.pop();
                    }
                }
            });
    });
    println!("\nTop Polyalphabetic Candidates:");
    let top_poly_candidates = top_poly_candidates.lock().unwrap();
    for Reverse(candidate) in top_poly_candidates.iter() {
        println!(
            "Score: {:.8}, Substitution: {}, Alphabet: {}, Keyword: {}, Plaintext: '{}'",
            candidate.score,
            candidate.substitution,
            candidate.alphabet,
            candidate.keyword,
            candidate.plaintext
        );
    }
}
