// main.rs

use std::fs;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use decryption::{Candidate, track_top_candidates};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

mod transposition;
mod substitution;
mod key_generation;
mod scoring;
mod alphabet_analysis;
mod decryption;

use crate::key_generation::{generate_keywords, generate_alphabets, decimate_alphabet};

use crate::alphabet_analysis::{contains_keyword, score_alphabet};


fn main() {
    // Read the K4 ciphertext from a file
    let ciphertext = fs::read_to_string("k4_ciphertext.txt").expect("Unable to read file");

    let mut top_candidates: BinaryHeap<Reverse<Candidate>> = BinaryHeap::new();
    let top_n = 10; // Track top 10 results

    let mut print_counter = 0;
    let print_frequency = 100;

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
    let alphabets = generate_alphabets(1000);
    let keywords = generate_keywords(308915776, 6);


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
            let alphabet_score = score_alphabet(alphabet);
            let mut local_heap = BinaryHeap::new();

            keywords.iter().for_each(|keyword| {
                if contains_keyword(alphabet, keyword) {
                    // Skip output, as before
                }

                transposition_techniques.iter().enumerate().for_each(|(i, &transposition_fn)| {
                    substitution_techniques.iter().enumerate().for_each(|(j, &substitution_fn)| {
                        let transposed_text = transposition_fn(&ciphertext, keyword);
                        let decimated_alphabet = decimate_alphabet(alphabet, keyword);
                        let plaintext_candidate = substitution_fn(&transposed_text, &decimated_alphabet);
                        let score = scoring::score_text(&plaintext_candidate);
                        let candidate = Candidate::new(score, plaintext_candidate, i, j, k, keyword.clone());

                        // Maintain top_n candidates in the local heap
                        if local_heap.len() < top_n {
                            local_heap.push(Reverse(candidate));
                        } else {
                            let mut smallest = local_heap.peek_mut().unwrap();
                            if candidate.score > smallest.0.score {
                                *smallest = Reverse(candidate);
                            }
                        }
                    });
                });
            });

            // Lock and merge local heap into the global heap
            let mut global_heap = top_candidates_global.lock().unwrap();
            while let Some(candidate) = local_heap.pop() {
                if global_heap.len() < top_n {
                    global_heap.push(candidate);
                } else {
                    let mut smallest = global_heap.peek_mut().unwrap();
                    if candidate.0.score > smallest.0.score {
                        *smallest = candidate;
                    }
                }
            }
        });
    });

    // Print results from global heap
    let top_candidates = top_candidates_global.lock().unwrap();
    println!("\nTop Candidates Summary:");
    for Reverse(candidate) in top_candidates.iter() {
        println!("Score: {:.2}, Transposition: {}, Substitution: {}, Alphabet: {}, Keyword: {}, Plaintext: '{}'",
                 candidate.score, candidate.transposition, candidate.substitution, candidate.alphabet_index, candidate.keyword, candidate.plaintext);
    }
}