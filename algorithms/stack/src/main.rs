struct Stack<T> {
    data: Vec<T>,
    size: usize,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.size -= 1;
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.get(self.size - 1)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.get_mut(self.size - 1)
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn size(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }

        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }

        iterator
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn main() {
    println!("Hello, world!");
    basic();

    println!("(()) is {}", par_check("(())"));

    println!("(())) is {}", par_check("(()))"));

    let bstr = divide_by_two(64);
    println!("b-string:{}", bstr);

    let digits: Vec<char> = "0123456789ABCDEF".chars().collect();
    println!("{:?}", "ddd".to_string()+&'X'.to_string());


    println!("{:?}", base_converter(1000, 16));

    fn basic() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);

        println!("{:?}", s.pop());
    }
}

fn par_check(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;
            } else {
                let _r = stack.pop();
            }
        }

        index += 1;
    }

    balance && stack.is_empty()
}

fn divide_by_two(mut num: usize) -> String {
    let mut mem = Stack::new();

    while num > 0 {
        let rem = num % 2;
        mem.push(rem);
        num /= 2;
    }

    let mut bstr = "".to_string();
    while !mem.is_empty() {
        let bc = mem.pop().unwrap().to_string();
        bstr += &bc;
    }

    bstr
}

fn base_converter(mut num: usize, base: usize) -> String {
    let digits: Vec<char> = "0123456789ABCDEF".chars().collect();
    let mut base_str = "".to_string();
    let mut stack = Stack::new();
    while num > 0 {
        let rem:usize = num % base;
        stack.push(rem);
        num /= base;
    }

    while !stack.is_empty(){
        let rem = stack.pop().unwrap();
        base_str += &digits[rem].to_string();
    }

    base_str
}
