#[derive(Debug)]
struct SBox<T:std::fmt::Debug>(T);

impl<T:std::fmt::Debug> SBox<T>{
    fn new(x:T) -> Self{
        Self(x)
    }
}


use std::ops::Deref;
use std::ops::Drop;

impl<T:std::fmt::Debug> Deref for SBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

impl<T:std::fmt::Debug> Drop for SBox<T>{
    fn drop(&mut self){
        println!("SBox drop itself:{:?}", self)
    }
}

fn main(){
    let x = 10;
    let y = SBox(x);
    println!("x={x}");
    println!("y={}", *y);
}