//! Command-line interface for the password generator
//!
//! This module provides a CLI for generating passwords using the pwd_gen library.

use clap::Parser;
use pwd_gen;
use std::num::{NonZeroUsize,NonZeroU32};

/// CLI arguments for the password generator
#[derive(Parser)]
#[command(name = "Password Generator")]
#[command(about = "Generate a password with specified entropy.")]
struct Cli {
    /// Desired entropy in bits (between 1 and 1024)
    #[arg(
        short,
        long,
        default_value_t = 80,
        help = "Desired entropy in bits (between 1 and 1024)",
        value_parser = clap::value_parser!(u32).range(1..=1024)
    )]
    entropy: u32,

    /// Number of characters per block (between 1 and 100)
    #[arg(
        short,
        long,
        default_value_t = 4,
        help = "Number of characters per block (between 1 and 100)",
        value_parser = clap::value_parser!(u32).range(1..=100)
    )]
    block_size: u32,

    /// Block separator (must be a single character)
    #[arg(
        short,
        long,
        default_value_t = '-',
        help = "Block separator (must be a single character)"
    )]
    separator: char,
}

/// Main function to parse CLI arguments and generate a password
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let entropy = NonZeroU32::try_from(cli.entropy)?;
    let block_size = NonZeroUsize::try_from(cli.block_size as usize)?;

    let password = pwd_gen::gen_pwd(entropy, block_size, cli.separator);
    println!("{}", password);

    Ok(())
}