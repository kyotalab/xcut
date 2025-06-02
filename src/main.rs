use anyhow::{Context, Result};
use evalexpr::eval_boolean;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

use clap::Parser;
use xcut::Args;

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

pub fn extract_columns<'a>(fields: &'a [&'a str], indices: &'a [usize]) -> Vec<&'a str> {
    indices
        .iter()
        .filter_map(|&i| fields.get(i - 1).copied())
        .collect()
}

pub fn eval_regex_filter(caps: &regex::Captures, fields: &[&str], negate: bool) -> Result<bool> {
    let col_index: usize = caps[1].parse().context("Failed to parse column index")?;
    let pattern = &caps[2];
    let value = fields.get(col_index - 1).unwrap_or(&"");

    let regex =
        Regex::new(pattern).with_context(|| format!("Invalid regex in --filter: {}", pattern))?;
    let matched = regex.is_match(value);

    Ok(if negate { !matched } else { matched })
}

pub fn evaluate_expression_with_fields(expr: &str, fields: &[&str]) -> Result<bool> {
    let re = Regex::new(r"col\((\d+)\)").unwrap();
    let filled_expr = re.replace_all(expr, |caps: &regex::Captures| {
        let idx = caps[1].parse::<usize>().unwrap_or(0); // unwrap_or で安全対策
        format!(r#""{}""#, fields.get(idx - 1).unwrap_or(&""))
    });
    Ok(eval_boolean(&filled_expr)?)
}

pub fn read_lines(args: Args, reader: Box<dyn BufRead>) -> Result<()> {
    let re_pos_regex = Regex::new(r#"col\((\d+)\)\s*=~\s*"(.+?)""#)?;
    let re_neg_regex = Regex::new(r#"col\((\d+)\)\s*!~\s*"(.+?)""#)?;

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = match (&args.delim, &args.max_split) {
            (Some(delim), Some(n)) => line.splitn(*n, delim).collect(),
            (Some(delim), None) => line.split(delim).collect(),
            (None, _) => line.split_whitespace().collect(),
        };

        if let Some(expr) = &args.filter {
            if let Some(caps) = re_pos_regex.captures(expr) {
                if !eval_regex_filter(&caps, &fields, false)? {
                    continue;
                }
            } else if let Some(caps) = re_neg_regex.captures(expr) {
                if !eval_regex_filter(&caps, &fields, true)? {
                    continue;
                }
            } else {
                if !evaluate_expression_with_fields(expr, &fields)? {
                    continue;
                }
            }
        }

        let out_delim = args.out_delim.as_deref().unwrap_or(" ");
        if let Some(ref vec) = args.cols {
            let extracted_line = extract_columns(&fields, vec);
            println!("{}", extracted_line.join(out_delim));
        } else {
            println!("{}", fields.join(out_delim));
        }
    }
    Ok(())
}
