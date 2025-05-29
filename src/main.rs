use clap::Parser;
use xcut::Args;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
