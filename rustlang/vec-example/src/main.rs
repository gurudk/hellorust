#[derive(Debug)]
struct Variable {
    id: i32,
}

fn main() {
    println!("Hello, world!");
    let mut vs = Vec::new();
    let id = 33;
    vs.push(Variable { id });
    let vv = Variable { id: 44 };
    let mut vsmut = Vec::<*mut Variable>::new();
    let rp = Box::into_raw(Box::new(vv));
    unsafe {
        println!("{:?}", *rp);
    }
    vsmut.push(rp);
    unsafe {
        println!("{:?}", *vsmut[0]);

        let va = return_vec();
        println!("va:{:?}", *va[0]);
        println!("vstr:{:?}", return_str_vec());
    }
}

fn return_vec() -> Vec<*mut Variable> {
    let mut v = Vec::new();
    v.push(Box::into_raw(Box::new(Variable { id: 1 })));
    v
}

fn return_str_vec()->Vec<&'static str>{
    let mut v = Vec::new();
    v.push("asdfasdf");
    v
}
