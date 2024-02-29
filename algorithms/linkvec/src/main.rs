use std::fmt::Debug;

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

struct LinkVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug> LinkVec<T> {
    fn new(data: T) -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    fn push(&mut self, data: T) {
        let node = Node::new(data);

        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut curr = self.head.as_mut().unwrap();

            for _i in 0..self.size - 1 {
                curr = curr.next.as_mut().unwrap();
            }

            curr.next = Some(Box::new(node));
        }

        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.remove(self.size - 1)
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }
        let mut node;
        if 0 == index {
            node = self.head.take().unwrap();
            self.head = node.next.take();
        } else {
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }

            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }

        self.size -= 1;

        Some(node.data)
    }
}

fn main() {
    println!("Hello, world!");
}
