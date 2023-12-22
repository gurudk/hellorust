struct Variable<'a>{
    data:Option<f64>,
    grad:Option<f64>,
    creator:Option<Box<&'a dyn for<'c> Function<'c>>>,
}

trait Function<'a>{

    fn set_input<'b:'a>(&mut self, inp:Option<&'b Variable>);
    fn get_input(&self)-> Box<Option<&Variable>>;

    fn set_output(&mut self, oup:Box<Option<Variable<'a>>>);
    fn get_output(&self)-> Box<Option<Variable>>;
    fn forward(&mut self)->f64;
    fn get_executor(&mut self)->Box<dyn FnMut(&Variable) -> Box<Variable<'a>> + '_>;

}

struct Square<'a>{
    input:Option<&'a Variable<'a>>,
    output:Box<Option<Variable<'a>>>,
}

impl<'a> Function<'a> for Square<'a>{
    fn set_input<'b:'a>(&mut self, inp:Option<&'b Variable>){
        self.input = inp;
    }

    fn get_input(&self)->Box<Option<&Variable>>{
        Box::new(self.input)
    }

    fn set_output(&mut self, oup:Box<Option<Variable<'a>>>){
        self.output = oup;
    }

    fn get_output(&self)->Box<Option<Variable>>{
        self.output
    }

    fn forward(&mut self)->f64{
        let x = self.input.unwrap().data;
        x.unwrap() * x.unwrap()
    }

    fn get_executor(&mut self)->Box<dyn FnMut(&Variable) -> Box<Variable<'a>> +'_>{
        let func = |v:&Variable|->Box<Variable>{
            let x = self.forward();
            Box::new(Variable{data:Some(x), creator:None, grad:None})
        };

        Box::new(func)
    }
}


fn main(){
    let x = Variable{data:Some(3_f64),creator:None,grad:None};
    let mut s = Square{input:Some(&x), output:Box::new(None)};
    let mut square = s.get_executor();
    let mut ret = square(&x);
    // s.output = Some(&ret);
    // ret.creator = Some(Box::new(&s));
    println!("{}", ret.data.unwrap());

}

