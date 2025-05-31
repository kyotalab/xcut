use anyhow::Result;
use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
};

use clap::Parser;
use xcut::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    if args.input.is_none() {
        eprintln!("Reading from stdin... (press Ctrl+D to end input)");
    }

    let reader: Box<dyn BufRead> = if let Some(path) = args.input {
        Box::new(BufReader::new(File::open(path)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split_whitespace().collect();

        if let Some(ref vec) = args.cols {
            let extracted_line = extract_columns(&fields, vec);
            extracted_line.iter().for_each(|f| println!("{}", f));
        }
    }
    Ok(())
}

pub fn extract_columns<'a>(fields: &'a [&'a str], indices: &'a [usize]) -> Vec<&'a str> {
    indices
        .iter()
        .filter_map(|&i| fields.get(i - 1).copied())
        .collect()
}
