// key_generation.rs

use rand::Rng;

pub fn grid_key_generator() -> String {
    const GRID_SIZE: usize = 5;
    let mut rng = rand::thread_rng();
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            grid[i][j] = rng.gen_range(0..31);
        }
    }

    let mut key = String::new();
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            key.push((grid[i][j] + b'A') as char);
        }
    }
    key
}

pub fn astronomical_key_generator() -> String {
    // This is a placeholder implementation. Replace with actual astronomical data.
    let astronomical_data = "ASTRONOMICALDATAFORYOURKEY";
    astronomical_data.to_string()
}

pub fn generate_alphabets(n: usize, length: Option<usize>) -> Vec<String> {
    let base_alphabet = "NGHIJLMNQUVWXZKRYPTOSABCDEFGHIJL";
    let alphabet_length = length.unwrap_or(base_alphabet.len());

    (0..n)
        .map(|i| {
            base_alphabet
                .chars()
                .cycle()
                .skip(i % base_alphabet.len()) // Ensures cycling continues appropriately
                .take(alphabet_length)
                .collect()
        })
        .collect()
}

// key_generation.rs

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

pub fn generate_keywords_from_wordlist(wordlist_file: &str, length: usize) -> Vec<String> {
    let file = File::open(wordlist_file).expect("Unable to open wordlist file");
    let mut reader = BufReader::new(file);

    let total_words = reader.by_ref().lines().count();
    let estimated_keywords = total_words / 10; // Adjust the estimation factor as needed

    println!("Total words in the wordlist: {}", total_words);
    println!("Estimated number of keywords: {}", estimated_keywords);

    let mut keywords = Vec::with_capacity(estimated_keywords);

    reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to seek to the beginning of the file");

    let mut processed_words = 0;
    let progress_interval = total_words / 100; // Print progress every 1% of total words

    for line in reader.lines().filter_map(|line| line.ok()) {
        processed_words += 1;

        if line.len() == length {
            keywords.push(line.to_ascii_uppercase());
        }

        if processed_words % progress_interval == 0 {
            let progress = processed_words as f64 / total_words as f64 * 100.0;
            println!("Keyword generation progress: {:.2}%", progress);
        }
    }

    println!(
        "Keyword generation completed. Generated {} keywords.",
        keywords.len()
    );

    keywords
}

pub fn decimate_alphabet(alphabet: &str, keyword: &str, ciphertext_length: usize) -> String {
    let alphabet_len = alphabet.len();
    let keyword_len = keyword.len();
    let mut decimated_alphabet = String::with_capacity(ciphertext_length);

    for i in 0..ciphertext_length {
        let index = (i * keyword_len) % alphabet_len;
        decimated_alphabet.push(alphabet.chars().nth(index).unwrap());
    }

    decimated_alphabet
}

pub fn generate_keywords(n: usize, length: usize) -> Vec<String> {
    let alphabet = "NGHIJLMNQUVWXZKRYPTOSABCDEFGHIJL";

    (0..n)
        .map(|_| {
            (0..length)
                .map(|_| {
                    alphabet
                        .chars()
                        .nth(rand::thread_rng().gen_range(0..31))
                        .unwrap()
                })
                .collect()
        })
        .collect()
}
