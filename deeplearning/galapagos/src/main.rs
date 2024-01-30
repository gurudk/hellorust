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
                let x = (*func).get_input();
                (*x).grad = Some((*func).backward(self.grad.unwrap()));
                (*x).backward();
            }
        }
    }
}

trait Function {
    fn set_input(&mut self, inp: *mut Variable);
    fn get_input(&self) -> *mut Variable;
    fn set_output(&mut self, oup: *mut Variable);
    fn get_output(&self) -> *mut Variable;
    fn forward(&mut self, x: *mut Variable) -> *mut Variable;
    fn backward(&mut self, gy: f64) -> f64;
    fn execute(&mut self, x: *mut Variable) -> *mut Variable;
}

#[derive(Debug)]
struct Square {
    input: *mut Variable,
    output: *mut Variable,
}

#[derive(Debug)]
struct Exp {
    input: *mut Variable,
    output: *mut Variable,
}

impl Function for Square {
    fn set_input(&mut self, inp: *mut Variable) {
        self.input = inp;
    }

    fn get_input(&self) -> *mut Variable {
        self.input
    }

    fn set_output(&mut self, oup: *mut Variable) {
        self.output = oup;
    }

    fn get_output(&self) -> *mut Variable {
        self.output
    }

    fn forward(&mut self, x: *mut Variable) -> *mut Variable {
        unsafe {
            self.input = x;
            self.output = Box::into_raw(Box::new(Variable {
                data: Some((*self.input).data.unwrap() * (*self.input).data.unwrap()),
                grad: None,
                creator: None,
            }));
            (*self.output).creator = Some(self);
            self.output
        }
    }

    fn backward(&mut self, gy: f64) -> f64 {
        unsafe {
            let x = self.input;
            let gx: f64 = 2.0 * (*x).data.unwrap() * gy;
            (*x).grad = Some(gx);
            gx
        }
    }

    fn execute(&mut self, x: *mut Variable) -> *mut Variable {
        self.forward(x)
    }
}

impl Function for Exp {
    fn set_input(&mut self, inp: *mut Variable) {
        self.input = inp;
    }

    fn get_input(&self) -> *mut Variable {
        self.input
    }

    fn set_output(&mut self, oup: *mut Variable) {
        self.output = oup;
    }

    fn get_output(&self) -> *mut Variable {
        self.output
    }

    fn forward(&mut self, x: *mut Variable) -> *mut Variable {
        unsafe {
            self.input = x;
            self.output = Box::into_raw(Box::new(Variable {
                data: Some((*self.input).data.unwrap().exp()),
                grad: None,
                creator: None,
            }));
            (*self.output).creator = Some(self);
            self.output
        }
    }

    fn backward(&mut self, gy: f64) -> f64 {
        unsafe {
            let x = self.input;
            let gx: f64 = (*x).data.unwrap().exp() * gy;
            (*x).grad = Some(gx);
            gx
        }
    }

    fn execute(&mut self, x: *mut Variable) -> *mut Variable {
        self.forward(x)
    }
}

impl Square {
    fn new_with_input(input: *mut Variable) -> *mut Self {
        unsafe {
            Box::into_raw(Box::new(Self {
                input: input,
                output: std::ptr::null_mut(),
            }))
        }
    }
}

impl Exp {
    fn new_with_input(input: *mut Variable) -> *mut Self {
        unsafe {
            Box::into_raw(Box::new(Self {
                input: input,
                output: std::ptr::null_mut(),
            }))
        }
    }
}

fn square(input: *mut Variable) -> *mut Variable {
    unsafe {
        let f = Square::new_with_input(input);
        let out = (*f).execute(input);
        out
    }
}

fn exp(input: *mut Variable) -> *mut Variable {
    unsafe {
        let f = Exp::new_with_input(input);
        let out = (*f).execute(input);
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
