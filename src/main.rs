use clap::Parser;
use pwd_gen;

fn validate_separator(s: &str) -> Result<char, String> {
    s.chars()
        .next()
        .filter(|_| s.len() == 1)
        .ok_or_else(|| String::from("Separator must be a single character"))
}

#[derive(Parser)]
#[command(name = "Password Generator")]
#[command(about = "Generate a password with specified entropy.")]
struct Cli {
    #[arg(short, long, default_value_t = 80, help = "Desired entropy in bits")]
    entropy: u32,

    #[arg(short, long, default_value_t = 4, help = "Number of characters per block")]
    block_size: usize,

    #[arg(
        short, 
        long, 
        default_value = "-", 
        help = "Block separator (must be a single character)",
        value_parser = validate_separator
    )]
    separator: char,
}

fn main() {
    let cli = Cli::parse();

    let password = pwd_gen::gen_pwd(
        cli.entropy, 
        cli.block_size, 
        cli.separator
    );
    println!("{}", password);
}