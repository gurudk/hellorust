fn main() {
    println!("Hello, world!");
    let s;
    {
        let x =10;
        s = S{r: &x};
    }
    // println!("{:?}", s);
}

#[derive(Debug)]
struct S<'a>{
    r: &'a i32
}

struct D<'b>{
    s:S<'b>,
}