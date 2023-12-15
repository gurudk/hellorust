
const EPS:f64 = 1e-4_f64;

#[derive(Debug,Copy,Clone)]
struct Variable{
    data:f64,
}

impl Variable{
    pub fn new(value:f64)->Self{
        Variable{data:value}
    }
}


struct Square();
struct Exp();


trait Functionable{
    fn get_function(&self)-> Box< dyn Fn(Variable)->Variable +'_>{

        let func = move |v:Variable| {
            let x = v.data;
            let y = self.forward(x);
            Variable{data:y}
        };

        Box::new(func)
    }

    fn forward(&self, x:f64)->f64;
}

impl Functionable for Square{

    fn forward(&self, x:f64)->f64{
        x*x
    }
}

impl Functionable for Exp{
    fn forward(&self, x:f64)->f64{
        x.exp()
    }
}

type F1 = Box<dyn Fn(Variable)->Variable>;

fn numerical_difference(f:F1, x:Variable, eps:f64)->f64{
    let x0:&Variable = &Variable{data:(x.data-eps)};
    let x1:&Variable = &Variable{data:(x.data+eps)};
    let y0:&Variable = &f(*x0);
    let y1:&Variable = &f(*x1);

    (y1.data - y0.data)/(x1.data - x0.data)    
}

fn main() {
    println!("Hello, world!");
    let x:Variable = Variable::new(3f64);
    let s = Square();
    let square = s.get_function();
    let result = square(x);
    let exp = (Exp()).get_function();

    println!("result:{:?}", exp(result));
    println!("exp derive:{:?}", numerical_difference(exp, x, EPS));


}



