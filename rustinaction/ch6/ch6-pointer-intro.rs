static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a = 42;
    let b = &B;
    let c = &C;
    let bb = &b;

    println!("a: {}, b: {:p}, c: {:p}, bb:{:p}", a, b, c,bb);

    println!("-=-=-====================-++++++++++++++++++++++++");

    let a:i32 = 40;
    let b:Box<i32> = Box::new(60);
    let c = a + *b;
    println!("c:{}", c);

    println!("-=-=-====================-++++++++++++++++++++++++");

    let a = Box::new(1);
    let b = Box::new(2);
    let c = Box::new(3);

    let result1 = *a + *b + *c;
    drop(a);

    let d = Box::new(4);
    let result2 = *b + *c +*d;
    println!("result1:{}, result2:{}", result1, result2);

}