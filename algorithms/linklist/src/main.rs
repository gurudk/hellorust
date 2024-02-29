type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Link<T>,
}

#[derive(Debug)]
struct LinkList<T> {
    head: Link<T>,
    size: usize,
}

impl<T> LinkList<T> {
    fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, item: T) {
        let node = Box::new(Node {
            element: item,
            //take means get head next node points to this node
            next: self.head.take(),
        });

        self.head = Some(node);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.element
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
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

struct IntoIter<T>(LinkList<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        //iterator only provide iterate function, just a place to call pop
        self.0.pop()
    }
}

struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        //Option<&Node<T>> --> Option<&T>
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        //Option<&Node<T>> --> Option<&T>
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

fn main() {
    println!("Hello, world!");

    let mut ll = LinkList::new();
    ll.push(1);

    println!("{:?}", ll);

    ll.push(2);

    println!("{:?}", ll);

    for item in ll.iter() {
        println!("{:?}", item);
    }

    for item in ll.iter_mut() {
        *item += 2;
    }

    println!("{:?}", ll);
}
