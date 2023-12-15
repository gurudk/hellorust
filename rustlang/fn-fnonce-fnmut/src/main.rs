
use std::ops::{FnOnce, FnMut, Fn};

struct Foo {}

impl FnOnce for Foo {
  type Output = (); 
  fn call_once(self, (arg,): (String,)) -> Self::Output {
    println!("Hello {}!", arg);
  }
}

impl FnMut for Foo {
  fn call_mut(&mut self, (arg,): (String,)) -> Self::Output {
    println!("Hello {}!", arg);
  }
}

impl Fn for Foo {
  fn call(&self, (arg,): (String,)) -> Self::Output {
    println!("Hello {}!", arg);
  }
}


fn main() {
    println!("Hello, world!");
    let foo = Foo();
}
