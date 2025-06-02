use clap::Parser;
use std::path::PathBuf;

/// xcut - An extended cut command with filtering and formatting capabilities.
#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the input file. Reads from stdin if not specified.
    #[arg(short = 'i', long)]
    pub input: Option<PathBuf>,

    /// Filter expression to match lines. Supports regex and boolean logic.
    ///
    /// Examples:
    ///
    /// - col(3) == "INFO"
    ///
    /// - col(4) =~ "^CPU"
    ///
    /// - col(3) !~ "DEBUG" && col(4) =~ "error"
    #[arg(short = 'f', long)]
    pub filter: Option<String>,

    /// List of column numbers to output (1-based index).
    ///
    /// Example: --cols 1,3
    #[arg(short = 'c', long, value_delimiter = ',')]
    pub cols: Option<Vec<usize>>,

    /// Delimiter used to split each line into columns.
    /// Default is whitespace.
    #[arg(long)]
    pub delim: Option<String>,

    /// Maximum number of splits to perform when using --delim.
    /// Useful to preserve trailing content in the last field.
    #[arg(long, value_name = "N")]
    pub max_split: Option<usize>,

    /// Delimiter used to join output fields.
    /// Default is a space.
    #[arg(long)]
    pub out_delim: Option<String>,

    /// Path to output file. Appends to the file if it exists.
    /// Defaults to stdout.
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Skip the first line (e.g. header in CSV files).
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_header: Option<bool>,

    /// Output only the first N lines (like `head`).
    #[arg(long)]
    pub head: Option<usize>,

    /// Output only the last N lines (like `tail`).
    #[arg(long)]
    pub tail: Option<usize>,
}
