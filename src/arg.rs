use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(name = "xcut", version = "0.1", author = "kyotalab")]
pub struct Args {
    #[arg(short = 'i', long = "input")]
    pub input: Option<PathBuf>,
    #[arg(short = 'f', long = "filter")]
    pub filter: Option<String>,
    // #[arg(short = 'p', long = "print")]
    // pub print: Option<String>,
    #[arg(short = 'c', long = "cols", value_delimiter = ',')]
    pub cols: Option<Vec<usize>>,
    #[arg(short = 'd', long = "delim")]
    pub delim: Option<String>,
    #[arg(short = 'm', long = "max-split")]
    pub max_split: Option<usize>,
    #[arg(long = "out-delim")]
    pub out_delim: Option<String>,
    #[arg(long = "output")]
    pub output: Option<PathBuf>,
    #[arg(long = "no-header", action = clap::ArgAction::SetTrue)]
    pub no_header: Option<bool>,
    #[arg(long = "head")]
    pub head: Option<usize>,
    #[arg(long = "tail")]
    pub tail: Option<usize>,
}
