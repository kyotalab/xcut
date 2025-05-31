use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[arg(short = 'i', long = "input")]
    pub input: Option<PathBuf>,
    #[arg(short = 'f', long = "filter")]
    pub filter: Option<String>,
    #[arg(short = 'p', long = "print")]
    pub print: Option<String>,
    #[arg(short = 'c', long = "cols", value_delimiter = ',')]
    pub cols: Option<Vec<usize>>,
    // 以下オプションは後で定義する
    // --delim, --no-header, --output, --limit, etc
}
