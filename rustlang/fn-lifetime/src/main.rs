fn main() {
    let ee = e{f:33};
    let cc = c{value:ee.clone()};
    diff_lifetime(cc, ee.clone());
}

fn diff_lifetime(p1:c,p2:e) ->i32{
    p1.value.f
}


struct c{
    value:e
}

#[derive(Clone)]
struct e{
    f:i32
}
