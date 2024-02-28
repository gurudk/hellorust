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
}

struct IntoIter<T>(LinkList<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        //iterator only provide iterate function, just a place to call pop
        self.0.pop()
    }
}

fn main() {
    println!("Hello, world!");

    let mut ll = LinkList::new();
    ll.push(1);

    println!("{:?}", ll);

    ll.push(2);

    println!("{:?}", ll);
}
