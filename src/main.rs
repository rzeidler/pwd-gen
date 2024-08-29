use clap::Parser;
use pwd_gen;

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

fn main() {
    let cli = Cli::parse();
    let password = pwd_gen::gen_pwd(cli.entropy, cli.block_size as usize, cli.separator);
    println!("{}", password);
}
