pub fn extract_columns<'a>(fields: &'a [&'a str], indices: &'a [usize]) -> Vec<&'a str> {
    indices
        .iter()
        .filter_map(|&i| fields.get(i - 1).copied())
        .collect()
}
