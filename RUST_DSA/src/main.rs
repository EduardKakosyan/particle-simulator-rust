//! DSA in Rust
/// Building algorithms in Rust to learn both DSA and Rust at the same time
mod algorithms;

use algorithms::anagram::{anagram_solution1, anagram_solution2};

fn main() {
    // Example usage of anagram algorithms
    let s1 = "listen";
    let s2 = "silent";

    println!("Are '{}' and '{}' anagrams?", s1, s2);
    println!("Solution 1: {}", anagram_solution1(s1, s2));
    println!("Solution 2: {}", anagram_solution2(s1, s2));
}
