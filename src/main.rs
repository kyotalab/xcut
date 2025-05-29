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
        fields.iter().for_each(|&f| println!("{}", f));
    }

    // println!("{:?}", args.clone());
    Ok(())
}
