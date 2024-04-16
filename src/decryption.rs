use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Candidate {
    pub score: f64,
    pub plaintext: String,
    pub transposition: usize,
    pub substitution: usize,
    pub alphabet_index: usize,
    pub keyword: String,
}

impl Candidate {
    pub fn new(score: f64, plaintext: String, transposition: usize, substitution: usize, alphabet_index: usize, keyword: String) -> Self {
        Self { score, plaintext, transposition, substitution, alphabet_index, keyword }
    }
}

impl Eq for Candidate {}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub fn track_top_candidates(candidates: &mut BinaryHeap<Reverse<Candidate>>, candidate: Candidate, top_n: usize) {
    // Track all candidates, regardless of score.
    candidates.push(Reverse(candidate));
    if candidates.len() > top_n {
        candidates.pop();
    }
}