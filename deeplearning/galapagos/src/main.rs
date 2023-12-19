
const EPS:f64 = 1e-4_f64;

#[derive(Debug,Copy,Clone)]
struct Variable{
    data:f64,
    grad:Option<f64>,
}

impl Variable{
    pub fn new(value:f64, grad:Option<f64>)->Self{
        Variable{data:value,grad:grad}
    }
}


struct Square{
    input:Variable,
}

#[derive(Debug)]
struct Exp{
    input:Variable,
}


impl Drop for Exp{
    fn drop(&mut self){
        println!("Dropping exp:{:?}", self);
    }
}


trait Functionable{

    fn get_function<'a>(&'a self)-> Box< dyn Fn(Variable)->Variable+'a>;
    fn forward(& self, x:Variable)->f64;
    fn backward(&self, gy:Variable)->f64;
}

impl Functionable for Square{

    fn get_function<'a>(&'a self)-> Box< dyn Fn(Variable)->Variable+'a>{
         let func = move |v:Variable| {

            let y = self.forward(v);
            Variable::new(y,None)
        };

        Box::new(func)
    }


    fn forward(& self, x:Variable)->f64{
        x.data*x.data
    }

    fn backward(&self, gy:Variable)->f64{
        let x = self.input.data;
        let dy = gy.data; 
        2_f64*x*dy 
    }
}

impl Functionable for Exp{

    fn get_function<'a>(&'a self)-> Box< dyn Fn(Variable)->Variable + 'a>{

        let func = move |v:Variable| {
            let y = self.forward(v);
            Variable::new(y,None)
        };

        Box::new(func)
    }

    fn forward(& self, x:Variable)->f64{
        x.data.exp()
    }

    fn backward(&self, gy:Variable)->f64{
        let x = self.input.data;
        x.exp()*gy.data
    }
}

fn numerical_difference(x:Variable, f:Box<dyn Fn(Variable)->Variable + '_>, eps:f64)-> f64{
    let x0:&Variable = &Variable::new(x.data-eps, None);
    let x1:&Variable = &Variable::new(x.data+eps, None);
    let y0:&Variable = &f(*x0);
    let y1:&Variable = &f(*x1);

    (y1.data - y0.data)/(x1.data - x0.data)    
}

fn main() {
    println!("Hello, world!");
    let x:Variable = Variable::new(3f64, None);

    let s = Square{input:x.clone()};
    let square = s.get_function();
    let result = square(x.clone());

    let ee = Exp{input:x.clone()};
    let exp = ee.get_function();

    // println!("result:{:?}", exp(result));
    // println!("exp derive:{:?},{:?}", ee, exp(x.clone()));

    println!("exp derive:{:?}", numerical_difference(x.clone(),exp, EPS));

    println!("{:?}, {:?}", ee, ee.input);


}



