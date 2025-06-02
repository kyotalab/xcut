use std::process::Command;

fn main() {
    let output = Command::new("target/debug/xcut")
        .args([
            "--input", "tests/fixtures/input1.txt",
            "--filter", r#"col(3) =~ "^INFO""#,
            "--cols", "3,4",
            "--delim", " ",
            "--max-split", "4"
        ])
        .output()
        .expect("failed to execute xcut with regex");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
