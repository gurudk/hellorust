use std::fmt::Debug;

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
struct LinkVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug> LinkVec<T> {
    fn new() -> Self {
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

    fn insert(&mut self, index: usize, item: T) {
        let mut idx = index;
        if index > self.size {
            idx = self.size;
        }

        let mut node = Node::new(item);

        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else if 0 == index {
            node.next = self.head.take(); //head节点指向的下一个节点变量为None
            self.head = Some(Box::new(node));
        } else {
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            node.next = curr.next.take();
            curr.next = Some(Box::new(node));
        }

        self.size += 1;
    }

    fn append(&mut self, other: &mut Self) {
        while let Some(node) = other.head.as_mut().take() {
            self.push(node.data);
            other.head = node.next.take();
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

struct IntoIter<T: Copy + Debug>(LinkVec<T>);
impl<T: Copy + Debug> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        } else {
            return self.0.remove(0);
        }
    }
}

struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            print_type_of(&node);
            self.next = node.next.as_deref(); //Box<Node> ---> & Node
            &node.data
        })
    }
}

struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            print_type_of(&node);
            self.next = node.next.as_deref_mut(); //Box<Node> ---> & Node
            &mut node.data
        })
    }
}

fn main() {
    println!("Hello, world!");

    let mut lv = LinkVec::new();
    lv.push(1);
    lv.push(2);
    lv.push(3);
    lv.insert(0, 8);

    println!("{:?}", lv);

    // for item in lv.into_iter() {
    //     println!("{:?}", item);
    // }
    for item in lv.iter() {
        println!("{:?}", item);
    }

    for item in lv.iter_mut() {
        *item *= 8;
    }
    
    println!("{:?}", lv);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
