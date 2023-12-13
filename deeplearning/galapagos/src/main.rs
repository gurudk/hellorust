
use ndarray::prelude::*;
use ndarray::{Array, IxDyn, Ix1,Ix2};

#[derive(Debug)]
struct Variable{
    data:ndarray::Array<f64, Ix1>,
}

impl Variable{
    pub fn new(value:Array<f64,Ix1>)->Self{
        Variable{data:value}
    }
}

struct Function(Variable);

trait Callable{
    fn call(variable:Variable)-> dyn Fn(Variable)->Variable ;
}


fn main() {
    println!("Hello, world!");
    let v:Variable = Variable::new(Array::range(1.0f64,10.5f64, 1.0f64));
    println!("v:{:?}", v);
    for d in v.data.into_iter(){
        println!("{}",d);
    }
}
