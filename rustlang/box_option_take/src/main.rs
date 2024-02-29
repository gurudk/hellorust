fn main() {
    println!("Hello, world!");
    let mut x = Some(Box::new(&5));

    let y = x.take();
    println!("{:?}:{:?}", x, y);
    print_type_of(&x);
    print_type_of(&y);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn print_type_of1<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}
