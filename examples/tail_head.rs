use std::process::Command;

fn main() {
    let output = Command::new("target/debug/xcut")
        .args([
            "--input", "tests/fixtures/input1.txt",
            "--cols", "3,4",
            "--delim", " ",
            "--max-split", "4",
            "--head", "3"
        ])
        .output()
        .expect("failed to extract head rows");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
