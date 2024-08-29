
use rand::{distributions::Alphanumeric, prelude::*};

pub fn gen_pwd(entropy: u32, block_size: usize, block_sep: char) -> String {
    let len_alphabet: usize = 62;

    let bits_per_block = (block_size as f64) * (len_alphabet as f64).log2();
    let n_blocks = (entropy as f64 / bits_per_block).ceil() as usize;

    let mut rng = thread_rng();
    let blocks: Vec<String> = (0..n_blocks).map(|_| {
        (0..block_size)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect()
    }).collect();
    blocks.join(&block_sep.to_string())
}