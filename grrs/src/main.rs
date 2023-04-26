#![allow(unused)]

use std::fs;

use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contains it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    //# The parse method is meant to be used in your main function
    let args = Cli::parse();

    // let content = fs::read_to_string(&args.path).expect("Could not read the file");

    // let content = match fs::read_to_string(&args.path) {
    //     Ok(x) => x,
    //     Err(err) => { return Err(err.into()); }
    // };

    //# The ? operator gives the same functionality that the match statement above
    // let content = fs::read_to_string(&args.path)?;

    let path = args.path;
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Could not read the file `{:?}`", path))?;

    // Read lines
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    return Ok(());
}
