use anyhow::Result;
use clap::Parser;
use indexmap::IndexMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser, Debug)]
struct ArgParser {
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

type CounterFn = fn(&str) -> usize;

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
    let mut args = ArgParser::parse();

    let mut output_parts: Vec<String> = vec![];

    // set default args if no args provided
    // TODO: conditional defaults with arg parser builder module?
    if !args.bytes && !args.lines && !args.words && !args.chars {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut input: Vec<u8> = vec![];
    let mut output_filename = None;

    // read from stdin if no filename
    // TODO: extract function
    if let Some(filename) = args.filename {
        let _ = File::open(filename.clone())?.read_to_end(&mut input);
        output_filename = Some(filename);
    } else {
        let _ = io::stdin().read_to_end(&mut input);
    }

    let contents = String::from_utf8(input)?;

    // TODO: use builder for arg parsing to be able to reference argument by strings
    // always return in the following order: lines, words, characters, bytes to match wc output
    let mut arg_function_mapping: IndexMap<&str, (bool, CounterFn)> = IndexMap::new();
    arg_function_mapping.insert("lines", (args.lines, count_file_lines));
    arg_function_mapping.insert("words", (args.words, count_file_words));
    arg_function_mapping.insert("chars", (args.chars, count_file_chars));
    arg_function_mapping.insert("bytes", (args.bytes, count_file_bytes));

    arg_function_mapping
        .into_iter()
        .for_each(|(_arg, (should_execute, function))| {
            if should_execute {
                output_parts.push(function(&contents).to_string());
            }
        });

    if let Some(filename) = output_filename {
        output_parts.push(filename);
    }

    // TODO: output formatter
    let output = output_parts.join(" ");
    println!("{output}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // bytes
    #[test]
    fn count_file_bytes_returns_correct_value() {
        let result = count_file_bytes("This is some test input");
        assert_eq!(result, 23);
    }

    // lines
    #[test]
    fn count_file_lines_returns_correct_value_for_single_line() {
        let result = count_file_lines("This input is all on a single line");
        assert_eq!(result, 1);
    }

    #[test]
    fn count_file_lines_returns_correct_value_for_multiple_lines() {
        let result = count_file_lines("This input is\nspread over\nmultiple\nlines");
        assert_eq!(result, 4);
    }

    // words
    #[test]
    fn count_file_words_returns_correct_value_for_non_empty_input() {
        let result = count_file_words("Input containing multiple words");
        assert_eq!(result, 4);
    }

    #[test]
    fn count_file_words_returns_correct_value_for_empty_input() {
        let result = count_file_words("");
        assert_eq!(result, 0);
    }

    // chars
    #[test]
    fn count_file_chars_returns_correct_value_for_empty_input() {
        let result = count_file_chars("Input containing many characters");
        assert_eq!(result, 32);
    }
}
