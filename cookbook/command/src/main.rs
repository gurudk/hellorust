use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::str;

fn main() {
    let dot = Command::new("dot") // `ps` command...
        .arg("-Tdot")
        .arg("tree.gv") // with argument `axww`...
        .stdout(Stdio::piped()) // of which we will pipe the output.
        .spawn() // Once configured, we actually spawn the command...
        .unwrap(); // and assert everything went right.
    let grep_child_one = Command::new("gvpr")
        .arg("-c")
        .arg("-f")
        .arg("btree.gvpr")
        .stdin(Stdio::from(dot.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child_two = Command::new("neato")
        .arg("-n")
        .arg("-Tpng")
        .stdin(Stdio::from(grep_child_one.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_child_two.wait_with_output().unwrap();
    let mut file = File::create("autowrite.png");
    file.unwrap().write_all(output.stdout.as_slice());
    // let output = grep_child_two.wait_with_output().unwrap();
    // let result = str::from_utf8(&output.stdout).unwrap();
    // println!("{}", result);
}
