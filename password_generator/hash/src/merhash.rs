//! Mersenne hash
//!
/// calculate hash value with mersenne prime 127
///
/// # Example
/// use hash::merhash::mersenne_hash;
///
/// let seed = String::from("jdxjp");
/// let hash = mersenne_hash(&seed);
pub fn mersenne_hash(seed: &str) -> usize {
    let mut hash: usize = 0;

    for (i, c) in seed.chars().enumerate() {
        hash += (i + 1) * (c as usize);
    }

    (hash % (127_usize.pow(3))) - 1
}
