use anyhow::Result;
use regex::Regex;
use std::io::BufRead;

use crate::Args;
use crate::{eval_regex_filter, evaluate_expression_with_fields, extract_columns};

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
