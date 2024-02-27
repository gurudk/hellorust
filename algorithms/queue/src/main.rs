#[derive(Debug)]
struct Queue<T> {
    data: Vec<T>,
    cap: usize,
}

impl<T> Queue<T> {
    fn new(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            cap: cap,
        }
    }

    fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            return Err("No place avaialable!".to_string());
        }
        self.data.insert(0, item);
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    fn is_full(&self) -> bool {
        self.data.len() == self.cap
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.data.pop()
    }

    fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&mut self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item)
        }

        iterator
    }

    fn size(&self)->usize{
        self.data.len()
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }

        iterator
    }
}

struct IntoIter<T>(Queue<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.data.remove(0))
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.remove(0))
        }
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.remove(0))
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut q = Queue::new(5);
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(66);
    q.enqueue(44);
    p::<i32>(&q);
    let e = q.dequeue();
    p::<i32>(&q);
    q.clear();
    p::<i32>(&q);

    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    q.enqueue(4);

    let sum1 = 10;
    let sum2 = q.iter().sum::<i32>();
    assert_eq!(sum1, sum2);

    p::<i32>(&q);

    let mut added = 0;

    for item in q.iter_mut() {
        *item += 1;
        added += 1;
    }

    let sum3 = q.iter().sum::<i32>();

    assert_eq!(sum1 + added, sum3);


    let names = vec!["a","b","lisi","zhangsan", "Tom", "Jerry","Sam"];
    let survival = hot_potato(names, 5);
    println!("survival:{}", survival);
}

fn p<T: std::fmt::Debug>(q: &Queue<T>) {
    println!("{:?}", q);
}

fn hot_potato(mut names:Vec<&str>, times:usize)->&str{
    let mut q = Queue::new(names.len());
    while !names.is_empty(){
        q.enqueue(names.pop().unwrap());
    }

    p::<&str>(&q);

    while q.size() > 1{
        for _i in 0..times-1{
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }
        p::<&str>(&q);
        let _rm = q.dequeue();
        println!("delete:{}", _rm.unwrap());
    }

    q.dequeue().unwrap()
}
