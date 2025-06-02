use std::process::Command;

fn main() {
    let output = Command::new("target/debug/xcut")
        .args([
            "--input", "tests/fixtures/input1.txt",
            "--cols", "2,4",
            "--delim", " ",
            "--max-split", "4",
            "--out-delim", ","
        ])
        .output()
        .expect("failed to generate CSV output");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
