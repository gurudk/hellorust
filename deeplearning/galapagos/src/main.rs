struct Variable{
    data:Option<f64>,
    creator:Option<Box<dyn for<'c> Function<'c>>>,
}

trait Function<'a>{

    fn set_input<'b:'a>(&mut self, inp:Option<&'b Variable>);
    fn get_input(&self)-> Box<Option<&Variable>>;
}

struct Square<'a>{
    input:Option<&'a Variable>,
}

impl<'a> Function<'a> for Square<'a>{
    fn set_input<'b:'a>(&mut self, inp:Option<&'b Variable>){
        self.input = inp;
    }

    fn get_input(&self)->Box<Option<&Variable>>{
        Box::new(self.input)
    }
}


fn main(){

}

