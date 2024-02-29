#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data: data,
            next: None,
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct ListStack<T> {
    size: usize,
    top: Link<T>,
}

impl<T: Clone> ListStack<T> {
    fn new() -> Self {
        Self { size: 0, top: None }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.top = None;
    }

    fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    fn peek(&self) -> Option<&T> {
        self.top.as_deref().map(|node| &node.data)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.top.as_deref_mut().map(|node| &mut node.data)
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.top.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.top.as_deref_mut(),
        }
    }
}

struct IntoIter<T>(ListStack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

fn main() {
    println!("Hello, world!");
    let mut ls: ListStack<i32> = ListStack::new();
    ls.push(3);

    println!("{:?}", ls);
    let p = ls.peek();

    println!("{:?}", p);

    let pm = ls.peek_mut();
    if let Some(data) = pm {
        *data = 666
    }
    println!("{:?}", ls);

    ls.push(444);
    ls.push(232);
    ls.push(888);

    for item in ls.iter() {
        println!("{:?}", item);
    }

    for item in ls.iter_mut() {
        *item = *item * 2;
    }

    println!("{:?}", ls);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
