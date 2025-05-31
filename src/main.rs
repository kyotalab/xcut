use anyhow::Result;
use evalexpr::eval_boolean;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
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
        // ここは`--delim` オプションを追加した時に、実装方法を変更する予定
        let fields: Vec<&str> = line.split_whitespace().collect();

        if let Some(expr) = args.filter.as_ref() {
            let re = Regex::new(r"col\((\d+)\)").unwrap();
            let filled_expr = re.replace_all(expr, |caps: &regex::Captures| {
                let idx = caps[1].parse::<usize>().unwrap();
                format!(r#""{}""#, fields.get(idx - 1).unwrap_or(&""))
            });
            if !eval_boolean(&filled_expr)? {
                continue;
            }
        }

        if let Some(ref vec) = args.cols {
            let extracted_line = extract_columns(&fields, vec);

            // ここは、区切り文字でjoinできるようにする
            println!("{}", extracted_line.join(" "));
        } else {
            println!("{}", line);
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
