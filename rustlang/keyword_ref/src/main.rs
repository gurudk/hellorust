#![allow(unused)]
fn main() {
let mut maybe_name = Some(String::from("Alice"));
// Using `ref`, the value is borrowed, not moved ...
match maybe_name {
    Some(ref mut n) => {n.push_str("xxx");println!("Hello, {n}")},
    _ => println!("Hello, world"),
}
// ... so it's available here!
println!("Hello again, {}", maybe_name.unwrap_or("world".into()));
}