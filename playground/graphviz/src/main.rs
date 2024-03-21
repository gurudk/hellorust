use std::io::{self, Write};
use std::process::Command;

fn main() {
    let c = "dot -Tdot tree.gv | gvpr -c -f btree.gvpr | neato -n -Tpng >tree.png";
    let output = Command::new(c)
        // .arg("--help")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
