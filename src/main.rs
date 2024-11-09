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

fn count_file_bytes(input: &str) -> Result<u64> {
    Ok(input.len() as u64)
}

fn count_file_chars(input: &str) -> Result<u64> {
    let char_count = input.chars().count();

    Ok(char_count as u64)
}

fn count_file_lines(input: &str) -> Result<u64> {
    let line_count = input.lines().count();
    Ok(line_count.try_into().unwrap())
}

fn count_file_words(input: &str) -> Result<u64> {
    let mut word_count = 0;

    for line in input.lines() {
        word_count += line.split_whitespace().count();
    }

    Ok(word_count.try_into().unwrap())
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
        let result = count_file_lines(&contents)?;
        output_parts.push(result.to_string());
    }

    if args.words {
        let result = count_file_words(&contents)?;
        output_parts.push(result.to_string());
    }

    if args.bytes {
        let result = count_file_bytes(&contents)?;
        output_parts.push(result.to_string());
    }

    if args.chars {
        let result = count_file_chars(&contents)?;
        output_parts.push(result.to_string());
    }

    if let Some(filename) = output_filename {
        output_parts.push(filename);
    }

    let output = output_parts.join(" ");
    println!("{output}");

    Ok(())
}
