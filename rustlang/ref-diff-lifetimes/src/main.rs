fn main() {
    println!("Hello, world!");
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S{x:&x, y:&y};
            r = s.x;
        }
    }
    println!("{}", r);
}

struct S<'a,'b>{
    x:&'a i32,
    y:&'b i32,
}

fn foo(x: &'static str) { }