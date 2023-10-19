use std::fmt;

struct Wrapper(Vec<String>);

//Similar java: Wrapper implements Display.....
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
}