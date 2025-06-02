use std::process::Command;

fn main() {
    let output = Command::new("target/debug/xcut")
        .args([
            "--input", "tests/fixtures/input1.txt",
            "--cols", "2,4",
            "--delim", " ",
            "--max-split", "4"
        ])
        .output()
        .expect("failed to execute xcut");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
