struct Cont<'a> {
    pub v: &'a i32
}

impl<'a> Cont<'a> {
    fn new(v: &'a i32) -> Cont {
        Cont {
            v: v
        }
    }
}

fn f<'a>() -> Cont<'a> {
    let v = Box::new(6);
    
    Cont::new(&v)
}

fn main() {
    let c = f();

    println!("{}", c.v);
}