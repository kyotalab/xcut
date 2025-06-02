use anyhow::Result;

pub fn extract_columns<'a>(fields: &'a [&'a str], indices: &'a [usize]) -> Result<Vec<&'a str>> {
    let extracted: Vec<&str> = indices
        .iter()
        .filter_map(|&i| fields.get(i - 1).copied())
        .collect();

    Ok(extracted)
}
