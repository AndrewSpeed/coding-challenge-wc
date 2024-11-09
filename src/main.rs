use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser)]
struct Args {
    filename: Option<String>,

    #[arg(short = 'c', long, help = "print the byte counts")]
    bytes: bool,

    #[arg(short = 'm', long, help = "print the character counts")]
    chars: bool,

    #[arg(short, long, help = "print the newline counts")]
    lines: bool,

    #[arg(short, long, help = "print the word counts")]
    words: bool,
}

fn count_file_bytes(input: &str) -> usize {
    input.len()
}

fn count_file_chars(input: &str) -> usize {
    input.chars().count()
}

fn count_file_lines(input: &str) -> usize {
    input.lines().count()
}

fn count_file_words(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, line| acc + line.split_whitespace().count())
}

fn main() -> Result<()> {
    let mut args = Args::parse();

    let mut output_parts: Vec<String> = vec![];

    // set default args if no args provided
    if !args.bytes && !args.lines && !args.words && !args.chars {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut input: Vec<u8> = vec![];
    let mut output_filename = None;

    // read from stdin if no filename
    if let Some(filename) = args.filename {
        let _ = File::open(filename.clone())?.read_to_end(&mut input);
        output_filename = Some(filename);
    } else {
        let _ = io::stdin().read_to_end(&mut input);
    }

    let contents = String::from_utf8(input)?;

    if args.lines {
        let result = count_file_lines(&contents);
        output_parts.push(result.to_string());
    }

    if args.words {
        let result = count_file_words(&contents);
        output_parts.push(result.to_string());
    }

    if args.bytes {
        let result = count_file_bytes(&contents);
        output_parts.push(result.to_string());
    }

    if args.chars {
        let result = count_file_chars(&contents);
        output_parts.push(result.to_string());
    }

    if let Some(filename) = output_filename {
        output_parts.push(filename);
    }

    let output = output_parts.join(" ");
    println!("{output}");

    Ok(())
}
