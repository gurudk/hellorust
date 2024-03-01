fn main() {
    println!("Hello, world!");

    let mut x = Some(Box::new(5));
    // print_type_of(&x);
    // print_type_of(&x.as_ref());
    // print_type_of(&x.as_deref());
    // print_type_of(&x.as_mut());
    // let mut zt = x.as_mut().take();
    // print_type_of(&zt);
    // let mut un = x.take().unwrap();
    // *un = 666;
    // print_type_of(&un);
    // print_type_of(&*un);
    x.map(|node|{
        println!("map type:");
        print_type_of(&node);
        node
    });
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
