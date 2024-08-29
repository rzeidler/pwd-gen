use rand::distributions::Slice;
use rand::rngs::OsRng;
use rand::Rng;

pub fn gen_pwd(entropy: u32, block_size: usize, block_sep: char) -> String {
    let alphabet: Vec<char> = ('a'..='z').chain('0'..='9').collect();
    let len_alphabet = alphabet.len();
    // println!("{}", String::from_iter(&alphabet));
    assert!(len_alphabet > 0, "Alphabet must be non-empty.");

    let alphabet_dist = Slice::new(&alphabet).unwrap();

    // Calculate number of blocks needed to achieve desired entropy.
    // TODO: Need to handle block_size = 0 with an error.
    let bits_per_block = (block_size as f64) * (len_alphabet as f64).log2();
    let n_blocks = (entropy as f64 / bits_per_block).ceil() as usize;

    let blocks: Vec<String> = (0..n_blocks)
        .map(|_| OsRng.sample_iter(&alphabet_dist).take(block_size).collect())
        .collect();
    blocks.join(&block_sep.to_string())
}
