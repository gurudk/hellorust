
fn main() {
    println!("Hello, world!");
    use std::cell::RefCell;

    let c = RefCell::new(5);

    // let five = c.into_inner();
    let old_value = c.replace(6);
    assert_eq!(old_value, 5);
    assert_eq!(c, RefCell::new(6));


    let cell = RefCell::new(5);
    let old_value = cell.replace_with(|&mut old| old + 2);
    assert_eq!(old_value, 5);
    assert_eq!(cell, RefCell::new(7));

    let c = RefCell::new(5);
    let d = RefCell::new(6);
    c.swap(&d);
    assert_eq!(c, RefCell::new(6));
    assert_eq!(d, RefCell::new(5));

    let c = RefCell::new(5);

    let borrowed_five = c.borrow();
    let borrowed_five2 = c.borrow();


    let c = RefCell::new(5);

    {
        let m = c.borrow_mut();
        assert!(c.try_borrow().is_err());
    }

    {
        let m = c.borrow();
        assert!(c.try_borrow().is_ok());
    }


    let c = RefCell::new("hello".to_owned());

    *c.borrow_mut() = "bonjour".to_owned();

    print_type(&c);
    print_type(&*(c.borrow_mut()));

    assert_eq!(&*c.borrow(), "bonjour");


    let c = RefCell::new(5);

    {
        let m = c.borrow();
        assert!(c.try_borrow_mut().is_err());
    }

    assert!(c.try_borrow_mut().is_ok());

    let c = RefCell::new(5);

    let ptr = c.as_ptr();

    unsafe {
        println!("{:?}", *ptr);
    }

    let mut c = RefCell::new(5);
    *c.get_mut() += 1;

    assert_eq!(c, RefCell::new(6));


    let c = RefCell::new(5);
    let five = c.take();

    assert_eq!(five, 5);
    assert_eq!(c.into_inner(), 0);
}

fn print_type<T>(_:&T){
    println!("{}", std::any::type_name::<T>());
}