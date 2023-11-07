fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;
    let c: i16 = 0b0100_0011_1100_0011 ;

    println!("a: {:016b} {}", a, a);     // <1>
    println!("b: {:016b} {}", b, b);     // <1>
    println!("b: {:016b} {}", c, c);
    println!("b: {:016b} {}", 15421, 15421);
}