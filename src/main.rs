use anyhow::Result;
use clap::Parser;
use std::fs::File;

#[derive(Parser)]
struct Args {
    filename: String,

    #[arg(short = 'c', long, help = "print the byte counts")]
    bytes: bool,
}

fn count_file_bytes(filename: &String) -> Result<u64> {
    let file = File::open(filename)?;
    let file_bytes = file.metadata()?.len();

    Ok(file_bytes)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut output_parts: Vec<String> = vec![];

    match args.bytes {
        true => output_parts.push(count_file_bytes(&args.filename)?.to_string()),
        false => {}
    };

    output_parts.push(args.filename);
    let output = output_parts.join(" ");
    println!("{output}");

    Ok(())
}
