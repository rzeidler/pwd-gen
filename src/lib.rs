use rand::distributions::Slice;
use rand::rngs::OsRng;
use rand::Rng;
use std::num::{NonZeroUsize,NonZeroU32};

pub fn gen_pwd(entropy: NonZeroU32, block_size: NonZeroUsize, block_sep: char) -> String {
    let alphabet: Vec<char> = ('a'..='z').chain('0'..='9').collect();
    let len_alphabet = alphabet.len();
    // println!("{}", String::from_iter(&alphabet));
    assert!(len_alphabet > 0, "Alphabet must be non-empty.");

    let alphabet_dist = Slice::new(&alphabet).unwrap();

    // Calculate number of blocks needed to achieve desired entropy.
    let bits_per_block = (block_size.get() as f64) * (len_alphabet as f64).log2();
    let n_blocks = (entropy.get() as f64 / bits_per_block).ceil() as usize;

    let blocks: Vec<String> = (0..n_blocks)
        .map(|_| OsRng.sample_iter(&alphabet_dist).take(block_size.get()).collect())
        .collect();
    blocks.join(&block_sep.to_string())
}
