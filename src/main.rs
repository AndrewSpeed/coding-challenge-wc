use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Parser)]
struct Args {
    filename: String,

    #[arg(short = 'c', long, help = "print the byte counts")]
    bytes: bool,

    #[arg(short = 'm', long, help = "print the character counts")]
    chars: bool,

    #[arg(short, long, help = "print the newline counts")]
    lines: bool,

    #[arg(short, long, help = "print the word counts")]
    words: bool,
}

fn count_file_bytes(file: &File) -> Result<u64> {
    let file_bytes = file.metadata()?.len();

    Ok(file_bytes)
}

fn count_file_chars(mut file: &File) -> Result<u64> {
    let mut file_contents = vec![];
    file.read_to_end(&mut file_contents)?;
    let char_count = String::from_utf8(file_contents)?.chars().count();

    Ok(char_count as u64)
}

fn count_file_lines(file: &File) -> Result<u64> {
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();
    Ok(line_count.try_into().unwrap())
}

fn count_file_words(file: &File) -> Result<u64> {
    let reader = BufReader::new(file);
    let mut word_count = 0;

    for line in reader.lines() {
        word_count += line?.split_whitespace().count();
    }

    Ok(word_count.try_into().unwrap())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut output_parts: Vec<String> = vec![];

    let file = File::open(args.filename.clone())?;

    match args.bytes {
        true => output_parts.push(count_file_bytes(&file)?.to_string()),
        false => {}
    };

    match args.lines {
        true => output_parts.push(count_file_lines(&file)?.to_string()),
        false => {}
    }

    match args.words {
        true => output_parts.push(count_file_words(&file)?.to_string()),
        false => {}
    }

    match args.chars {
        true => output_parts.push(count_file_chars(&file)?.to_string()),
        false => {}
    }

    output_parts.push(args.filename);
    let output = output_parts.join(" ");
    println!("{output}");

    Ok(())
}
