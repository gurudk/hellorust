struct Variable<'a>{
    data:Box<Option<f64>>,
    creator:Box<Option<&'a mut dyn Function>>,
}

trait Function{

    fn set_input(&mut self, inp:Box<Option<Variable>>);
    fn get_input(&self)-> Box<Option<Variable>>;
    fn set_output(&mut self, oup:Box<Option<Variable>>);
    fn get_output(&self)-> Box<Option<Variable>>;
    fn forward(&mut self);
    fn backward(&self);
}

struct Square<'a>{
    input:Box<Option<Variable<'a>>>,
    output:Box<Option<Variable<'a>>>,
}

impl<'a> Function for Square<'a>{
    fn get_input(&self)-> Box<Option<Variable>>{
        self.input
    }

    fn get_output(&self)-> Box<Option<Variable>>{
        self.output
    }

    fn set_input(&mut self, inp:Box<Option<Variable>>){
        self.input = inp;
    }

    fn set_output(&mut self, oup:Box<Option<Variable>>){
        self.output = oup;
    }

    fn forward(&mut self){
        let x = self.input.unwrap().data;
        self.output = Box::new(Some(Variable{
            data:Box::new(Some(x.unwrap()*x.unwrap())),
            creator:Box::new(Some(self)),
        }));
    }

    fn backward(&self){

    }
}


fn main(){

}

