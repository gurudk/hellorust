fn greet_world() {
    println!("Hello, world!");     // <1>

    let southern_germany = "Grüß Gott!";         // <2>
    let japan = "rust, 你好";                // <3>

    let regions = [southern_germany, japan];     // <4>

    for region in regions.iter() {               // <5>
        println!("{}", &region);             // <6>
    }
}

fn main() {
    greet_world();                               // <7>
}