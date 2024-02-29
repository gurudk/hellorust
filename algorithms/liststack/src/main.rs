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
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
