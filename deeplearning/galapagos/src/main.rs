struct Variable {
    data: Option<f64>,
    grad: Option<f64>,
    creator: Option<*mut dyn Function>,
}

trait Function {
    fn set_input(&mut self, inp: *mut Variable);
    fn get_input(&self) -> *mut Variable;
    fn set_output(&mut self, oup: *mut Variable);
    fn get_output(&self) -> *mut Variable;
    fn forward(&mut self) -> *mut Variable;
    fn execute(&mut self) -> *mut Variable;
}

struct Square {
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

    fn forward(&mut self) -> *mut Variable {
        unsafe {
            Box::into_raw(Box::new(Variable {
                data: Some((*self.input).data.unwrap() * (*self.input).data.unwrap()),
                grad: None,
                creator: None,
            }))
        }
    }

    fn execute(&mut self) -> *mut Variable {
        self.forward()
    }
}

fn main() {}
