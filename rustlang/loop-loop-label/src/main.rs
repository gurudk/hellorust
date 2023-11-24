fn main() {
    println!("Hello, world!");
    'a: loop {
        'a: loop {
            println!("inner loop");
            break 'a;
        }
        print!("outer loop");
        break 'a;
    }
}
