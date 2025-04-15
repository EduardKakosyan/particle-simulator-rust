//! DSA in Rust
/// Building algorithms in Rust to learn both DSA and Rust at the same time
/// # Problem 1 Anagram
/// The problem of checking if two strings are permutations of each other.
/// Permutation is defined as one string, s1, being a rearrangmenent of another
/// string, s2.
/// ## Assumptions
/// s1 and s2 are of same length
/// consist only of 26 lowercase letters.
/// ## Solution 1 - checking if one value is present in another
pub fn anagram_solution1(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let mut pos1: usize = 0;
    let mut pos2: usize;

    let mut is_anagram = true;
    let mut found: bool;

    for c in s1.chars() {
        vec1.push(c);
    }

    for c in s2.chars() {
        vec2.push(c);
    }

    while pos1 < s1.len() && is_anagram {
        pos2 = 0;
        found = false;
        while pos2 < s2.len() && !found {
            if vec1[pos1] == vec2[pos2] {
                found = true;
            } else {
                pos2 += 1;
            }
        }

        if found {
            vec2[pos2] = ' ';
        } else {
            is_anagram = false;
        }

        pos1 += 1;
    }

    is_anagram
}

/// ## Solution 2 - sort the strings in ascending order and check if they are the same
/// Big O of nlogn
pub fn anagram_solution2(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for c in s1.chars() {
        vec1.push(c);
    }
    for c in s2.chars() {
        vec2.push(c);
    }

    vec1.sort();
    vec2.sort();

    let mut is_anagram = true;
    let mut pos: usize = 0;

    while pos < vec1.len() && is_anagram {
        if vec1[pos] == vec2[pos] {
            pos += 1;
        } else {
            is_anagram = false;
        }
    }

    is_anagram
}
