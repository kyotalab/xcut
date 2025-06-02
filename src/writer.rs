use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn get_writer(output_path: &Option<PathBuf>) -> Result<Box<dyn Write>> {
    match output_path {
        Some(path) => {
            let file = OpenOptions::new().append(true).create(true).open(path)?;
            Ok(Box::new(file))
        }
        None => Ok(Box::new(io::stdout())),
    }
}
