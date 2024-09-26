use std::{
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

use anyhow::Result;
use bpe_tokenizer::BytePairEncoder;
use clap::{ArgAction, Parser};

/// A simple CLI for tokenizing text input using Byte Pair Encoding (BPE).
///
/// This application accepts text from stdin and emits BPE tokens to stdout.
#[derive(Parser)]
#[command(version, about, long_about, author)]
#[group(multiple = false)]
struct Cli {
    /// Use built-in small vocabulary (100k).
    #[arg(short, long, action = ArgAction::SetTrue)]
    small: Option<bool>,

    /// Use built-in medium vocabulary (320k) [default]
    #[arg(short, long, action = ArgAction::SetTrue, default_value = "true")]
    medium: Option<bool>,

    /// Use built-in large vocabulary (1M).
    #[arg(short, long, action = ArgAction::SetTrue)]
    large: Option<bool>,

    /// Path to custom BPE vocabulary file.
    #[arg(short, long, value_name = "FILE")]
    vocab: Option<PathBuf>,
}

fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Load vocabulary
    let bpe = if let Some(vocab_path) = cli.vocab.as_deref() {
        BytePairEncoder::new_from_file(
            vocab_path
                .to_str()
                .ok_or(anyhow::anyhow!("Invalid vocabulary path"))?,
        )?
    } else if cli.small.unwrap_or(false) {
        BytePairEncoder::new_default_small()?
    } else if cli.large.unwrap_or(false) {
        BytePairEncoder::new_default_large()?
    } else {
        BytePairEncoder::new_default_medium()?
    };

    // Create a buffer reader for stdin
    let reader = BufReader::new(io::stdin().lock());

    // Iterate over the lines of stdin while tokenizing and emitting to stdout
    for line in reader.lines().map_while(Result::ok) {
        bpe.tokenize_iter(&line).for_each(|t| {
            println!("{}", t);
        });
        io::stdout().flush()?;
    }

    // Success
    Ok(())
}
