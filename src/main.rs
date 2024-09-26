use std::{
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use bpe_tokenizer::BytePairEncoder;
use clap::{ArgAction, Parser};

/// A simple CLI for tokenizing text input using Byte Pair Encoding (BPE).
///
/// This application accepts text from stdin and emits BPE tokens to stdout.
#[derive(Parser)]
#[command(version, about, long_about, author)]
#[group(multiple = false)]
struct VocabularyOption {
    /// Use built-in small vocabulary (100k).
    #[arg(short, long, action = ArgAction::SetTrue)]
    small: bool,

    /// Use built-in medium vocabulary (320k) [default]
    #[arg(short, long, action = ArgAction::SetTrue, default_value_t = true)]
    medium: bool,

    /// Use built-in large vocabulary (1M).
    #[arg(short, long, action = ArgAction::SetTrue)]
    large: bool,

    /// Path to custom BPE vocabulary file.
    #[arg(short, long, value_name = "FILE")]
    vocab: Option<PathBuf>,
}

fn main() -> Result<()> {
    // Parse CLI arguments
    let options = VocabularyOption::parse();

    // Load vocabulary
    let bpe = match options.vocab {
        Some(ref vocab_path) => BytePairEncoder::new_from_file(&vocab_path.to_string_lossy())?,
        _ if options.small => BytePairEncoder::new_default_small()?,
        _ if options.large => BytePairEncoder::new_default_large()?,
        _ => BytePairEncoder::new_default_medium()?, // Default case is medium
    };

    // Buffer reader for stdin
    let reader = BufReader::new(io::stdin().lock());

    // Process each line, tokenize, and print tokens
    for line_result in reader.lines() {
        let line = line_result.context("Failed to read line from stdin")?;
        for token in bpe.tokenize_iter(&line) {
            println!("{}", token);
        }
        io::stdout().flush()?;
    }

    // Success
    Ok(())
}
