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
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// fn print_type_of1<T>(_: &T) {
//     println!("{}", unsafe { std::intrinsics::type_name::<T>() });
// }
