use clap::Parser;
use pwd_gen;
use std::num::{NonZeroUsize,NonZeroU32};

#[derive(Parser)]
#[command(name = "Password Generator")]
#[command(about = "Generate a password with specified entropy.")]
struct Cli {
    #[arg(
        short,
        long,
        default_value_t = 80,
        help = "Desired entropy in bits (between 1 and 1024)",
        value_parser = clap::value_parser!(u32).range(1..=1024)
    )]
    entropy: u32,

    #[arg(
        short,
        long,
        default_value_t = 4,
        help = "Number of characters per block (between 1 and 100)",
        value_parser = clap::value_parser!(u32).range(1..=100)
    )]
    block_size: u32,

    #[arg(
        short,
        long,
        default_value_t = '-',
        help = "Block separator (must be a single character)"
    )]
    separator: char,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let entropy = NonZeroU32::try_from(cli.entropy)?;
    let block_size = NonZeroUsize::try_from(cli.block_size as usize)?;
    
    let password = pwd_gen::gen_pwd(entropy, block_size, cli.separator);
    println!("{}", password);
    
    Ok(())
}
