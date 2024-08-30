//! Password generation library
//!
//! This library provides functionality to generate passwords with specified entropy,
//! block size, and block separator.

use rand::distributions::Slice;
use rand::rngs::OsRng;
use rand::Rng;
use std::num::{NonZeroUsize,NonZeroU32};

/// Generates a password with specified entropy, block size, and block separator.
///
/// # Arguments
///
/// * `entropy` - Desired entropy in bits (as a NonZeroU32)
/// * `block_size` - Number of characters in each block of the password (as a NonZeroUsize)
/// * `block_sep` - Character used to separate blocks in the password
///
/// # Returns
///
/// A String containing the generated password
///
pub fn gen_pwd(entropy: NonZeroU32, block_size: NonZeroUsize, block_sep: char) -> String {
    let alphabet: Vec<char> = ('a'..='z').chain('0'..='9').collect();
    let len_alphabet = alphabet.len();
    let alphabet_dist = Slice::new(&alphabet).unwrap();

    // Calculate number of blocks needed to achieve desired entropy.
    let bits_per_block = (block_size.get() as f64) * (len_alphabet as f64).log2();
    let n_blocks = ((entropy.get() as f64 / bits_per_block).ceil() as usize).max(1);

    // Generate password blocks
    let blocks: Vec<String> = (0..n_blocks)
        .map(|_| OsRng.sample_iter(&alphabet_dist).take(block_size.get()).collect())
        .collect();

    // Join the blocks and return password
    blocks.join(&block_sep.to_string())
}