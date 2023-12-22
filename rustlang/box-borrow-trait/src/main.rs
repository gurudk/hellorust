use std::borrow::BorrowMut;

trait Foo {}
struct Bar;
impl Foo for Bar {}

fn main() {
    let mut encryptor: Box<dyn Foo> = Box::new(Bar);

    // encrypt((*encryptor).borrow_mut());
    encrypt(encryptor.borrow_mut());
}

fn encrypt<'a>(encryptor: &mut (dyn Foo + 'a)) { }
// fn encrypt(encryptor: &mut dyn Foo) { }