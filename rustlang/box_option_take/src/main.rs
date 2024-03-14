fn main() {
    println!("Hello, world!");
    let mut x = Some(Box::new(&5));
    let mut z = Some(Box::new(5));

    let y = x.take();
    println!("{:?}:{:?}", x, y);
    print_type_of(&x);
    print_type_of(&y);

    match &z {
        Some(k) => {
            print_type_of(&k);
            print_type_of(&k.as_ref());
            print_type_of(&*k);
        }
        None => (),
    }
    let mut zz = z.as_deref_mut().unwrap();
    *zz = 222;
    // print_type_of(&z.as_deref());

    // print_type_of(&z);
    // print_type_of(&z.as_ref());
    let mut bb = &Box::new(444);
    let b = (*bb.as_ref() as i32).pow(2);
    println!("{:?}", b);
    print_type_of(&bb.as_ref());

    println!("=============test box reference ============");

    let ts = TestStruct {
        field1: 12121,
        field2: "test joint",
    };

    let tt = TestStruct {
        field1: 12121,
        field2: "test joint",
    };

    let bb = &Some(Box::new(ts));
    let r1 = &Some(tt);
    match &&bb {
        Some(k) => {
            println!("{:?}, {}", k, k.field1);
            print_type_of(&k);
        }
        None => (),
    };

    match r1 {
        Some(r) => {
            println!("r1.field1:{},r1.field2:{}", r.field1, r.field2);
            print_type_of(&r);
        }
        None => (),
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
struct TestStruct<'a> {
    field1: usize,
    field2: &'a str,
}

// fn print_type_of1<T>(_: &T) {
//     println!("{}", unsafe { std::intrinsics::type_name::<T>() });
// }
