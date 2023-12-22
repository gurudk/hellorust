struct Cont {
    pub v: Box<i32>
}

impl Cont {
    fn new(v: Box<i32>) -> Cont {
        Cont {
            v: v
        }
    }
}

fn f() -> Cont {
    let v = Box::new(6);
    Cont::new(v)
}

fn main() {
    let c = f();

    println!("{}", c.v);
}

//要把结构的属性设置为Box封装