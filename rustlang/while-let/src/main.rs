
fn main() {
    println!("Hello, world!");
    let mut v = vec![1,2,3,4,5,6];

    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}
