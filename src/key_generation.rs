
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

pub fn generate_alphabets(n: usize) -> Vec<String> {
    let mut alphabets = Vec::new();
    let base_alphabet = "NGHIJLMNQUVWXZKRYPTOSABCDEFGHIJL";

    for i in 0..n {
        let shifted_alphabet = base_alphabet.chars().cycle().skip(i).take(31).collect::<String>();
        alphabets.push(shifted_alphabet);
    }

    alphabets
}

pub fn generate_keywords(n: usize, length: usize) -> Vec<String> {
    let mut keywords = Vec::new();
    let alphabet = "NGHIJLMNQUVWXZKRYPTOSABCDEFGHIJL";

    for _ in 0..n {
        let keyword = (0..length).map(|_| alphabet.chars().nth(rand::thread_rng().gen_range(0..31)).unwrap()).collect::<String>();
        keywords.push(keyword);
    }

    keywords
}

pub fn decimate_alphabet(alphabet: &str, keyword: &str) -> String {
    let alphabet_len = alphabet.len();
    let keyword_len = keyword.len();
    let mut decimated_alphabet = String::with_capacity(alphabet_len);

    for i in 0..alphabet_len {
        let index = (i * keyword_len) % alphabet_len;
        decimated_alphabet.push(alphabet.chars().nth(index).unwrap());
    }

    decimated_alphabet
}