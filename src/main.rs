use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

use clap::Parser;
use xcut::Args;
use xcut::read_lines;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1)
    }
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    if args.input.is_none() {
        eprintln!("Reading from stdin... (press Ctrl+D to end input)");
    }

    let reader: Box<dyn BufRead> = if let Some(ref path) = args.input {
        Box::new(BufReader::new(File::open(path)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    read_lines(args, reader)?;

    Ok(())
}
