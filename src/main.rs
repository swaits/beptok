use std::{
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

use anyhow::Result;
use bpe_tokenizer::BytePairEncoder;
use clap::{Parser, ArgAction};

/// A simple CLI for tokenizing text input using Byte Pair Encoding (BPE).
///
/// This application accepts text from stdin and emits BPE tokens to stdout.
///
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

    // Load vocabulary from file
    let vocab = if let Some(vocab_path) = cli.vocab.as_deref() {
        match BytePairEncoder::new_from_file(vocab_path.to_str().unwrap()) {
            Ok(vocab) => vocab,
            Err(err) => {
                eprintln!("Error loading vocabulary file: {}", err);
                std::process::exit(1);
            }
        }
    } else if cli.small.unwrap_or(false) {
        match BytePairEncoder::new_default_small() {
            Ok(bpe) => bpe,
            Err(err) => {
                eprintln!("Error initializing default-small vocabulary file: {}", err);
                std::process::exit(1);
            }
        }
    } else if cli.large.unwrap_or(false) {
        match BytePairEncoder::new_default_large() {
            Ok(bpe) => bpe,
            Err(err) => {
                eprintln!("Error initializing default-large vocabulary file: {}", err);
                std::process::exit(1);
            }
        }
    } else {
        match BytePairEncoder::new_default_medium() {
            Ok(bpe) => bpe,
            Err(err) => {
                eprintln!("Error initializing default-medium vocabulary file: {}", err);
                std::process::exit(1);
            }
        }
    };

    // Create a buffer reader for stdin
    let reader = BufReader::new(io::stdin().lock());

    // Iterate over the lines of stdin, simultaneously tokenizing and emitting to stdout
    for line in reader.lines() {
        let line = line?;

        // Tokenize each line
        for token in vocab.tokenize_iter(&line) {
            // Emit token
            println!("{}", token);

            // Ensure output is flushed immediately
            io::stdout().flush()?;
        }
    }

    Ok(())
}
