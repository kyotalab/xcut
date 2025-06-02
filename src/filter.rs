use anyhow::{Context, Result};
use evalexpr::eval_boolean;
use regex::Regex;

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
