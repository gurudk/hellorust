use std::iter::zip;

#[derive(Debug)]
struct Variable {
    data: Option<f64>,
    grad: Option<f64>,
    creator: Option<*mut dyn Function>,
}

impl Variable {
    fn set_creator(&mut self, func: *mut dyn Function) {
        self.creator = Some(func)
    }

    fn backward(&mut self) {
        unsafe {
            if let Some(func) = self.creator {
                let xs = (*func).get_input();
                let grads = (*func).backward(self.grad.unwrap());
                for (x,grad) in zip(xs,grads) {
                    (*(*x)).grad = Some(grad);
                    (*(*x)).backward();
                }
            }
        }
    }
}

trait Function {
    fn set_input(&mut self, inputs: Vec<*mut Variable>);
    fn get_input(&self) -> & Vec<*mut Variable>;
    fn set_output(&mut self, outputs: *mut Variable);
    fn get_output(&self) -> *mut Variable;
    fn forward(&mut self, xs: Vec<*mut Variable>) -> *mut Variable;
    fn backward(&mut self, gy: f64) -> Vec<f64>;
    fn execute(&mut self, x: Vec<*mut Variable>) -> *mut Variable;
}

#[derive(Debug)]
struct Square {
    input: Vec<*mut Variable>,
    output: *mut Variable,
}

#[derive(Debug)]
struct Exp {
    input: Vec<*mut Variable>,
    output: *mut Variable,
}

impl Function for Square {
    fn set_input(&mut self, input: Vec<*mut Variable>) {
        self.input = input;
    }

    fn get_input(&self) -> & Vec<*mut Variable> {
        & self.input
    }

    fn set_output(&mut self, oup: *mut Variable) {
        self.output = oup;
    }

    fn get_output(&self) -> *mut Variable {
        self.output
    }

    fn forward(&mut self, xs: Vec<*mut Variable>) -> *mut Variable {
        unsafe {
            self.input = xs;
            let x = self.input[0];
            self.output = Box::into_raw(Box::new(Variable {
                data: Some((*x).data.unwrap() * (*x).data.unwrap()),
                grad: None,
                creator: None,
            }));
            (*self.output).creator = Some(self);
            self.output
        }
    }

    fn backward(&mut self, gy: f64) -> Vec<f64> {
        unsafe {
            let x = self.input[0];
            let gx: f64 = 2.0 * (*x).data.unwrap() * gy;
            (*x).grad = Some(gx);
            let mut ret = Vec::<f64>::new();
            ret.push(gx);
            ret
        }
    }

    fn execute(&mut self, xs: Vec<*mut Variable>) -> *mut Variable {
        self.forward(xs)
    }
}

impl Function for Exp {
    fn set_input(&mut self, input: Vec<*mut Variable>) {
        self.input = input;
    }

    fn get_input(&self) -> & Vec<*mut Variable> {
        & self.input
    }

    fn set_output(&mut self, oup: *mut Variable) {
        self.output = oup;
    }

    fn get_output(&self) -> *mut Variable {
        self.output
    }

    fn forward(&mut self, xs: Vec<*mut Variable>) -> *mut Variable {
        unsafe {
            self.input = xs;
            let x = self.input[0];
            self.output = Box::into_raw(Box::new(Variable {
                data: Some((*x).data.unwrap().exp()),
                grad: None,
                creator: None,
            }));
            (*self.output).creator = Some(self);
            self.output
        }
    }

    fn backward(&mut self, gy: f64) -> Vec<f64> {
        unsafe {
            let x = self.input[0];
            let gx: f64 = (*x).data.unwrap().exp() * gy;
            (*x).grad = Some(gx);
            let mut ret = Vec::<f64>::new();
            ret.push(gx);
            ret
        }
    }

    fn execute(&mut self, xs: Vec<*mut Variable>) -> *mut Variable {
        self.forward(xs)
    }
}

impl Square {
    fn new_with_input(input: *mut Variable) -> *mut Self {
        unsafe {
            let mut inputs = Vec::<*mut Variable>::new();
            inputs.push(input);
            Box::into_raw(Box::new(Self {
                input: inputs,
                output: std::ptr::null_mut(),
            }))
        }
    }
}

impl Exp {
    fn new_with_input(input: *mut Variable) -> *mut Self {
        unsafe {
            let mut inputs = Vec::<*mut Variable>::new();
            inputs.push(input);
            Box::into_raw(Box::new(Self {
                input: inputs,
                output: std::ptr::null_mut(),
            }))
        }
    }
}

fn square(input: *mut Variable) -> *mut Variable {
    unsafe {
        let mut inputs = Vec::<*mut Variable>::new();
        inputs.push(input);
        let f = Square::new_with_input(input);
        let out = (*f).execute(inputs);
        out
    }
}

fn exp(input: *mut Variable) -> *mut Variable {
    unsafe {
        let mut inputs = Vec::<*mut Variable>::new();
        inputs.push(input);
        let f = Exp::new_with_input(input);
        let out = (*f).execute(inputs);
        out
    }
}

fn main() {
    let input: *mut Variable = Box::into_raw(Box::new(Variable {
        data: Some(2_f64),
        grad: None,
        creator: None,
    }));

    // let s = Square::new_with_input(input);

    unsafe {
        let out = square(exp(input));

        let creator = (*out).creator.unwrap();

        let gy = 1.0_f64;
        (*out).grad = Some(1.0);
        (*out).backward();

        println!("input:{:?}", *input);
    }
}
