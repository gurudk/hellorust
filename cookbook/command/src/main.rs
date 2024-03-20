#![allow(unused)]
fn main() {
use std::process::Command;
use std::io::{self, Write};
let output = Command::new("dot")
    .arg("--help")
    .output()
    .expect("failed to execute process");

println!("status: {}", output.status);
io::stdout().write_all(&output.stdout).unwrap();
io::stderr().write_all(&output.stderr).unwrap();

assert!(output.status.success());
}