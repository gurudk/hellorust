use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacroPro;

#[derive(HelloMacroPro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}