fn main() {
    println!("Hello, world!");
    let mut avt = AvlTree::new(5);
    avt.insert(3);
    avt.insert(5);
    avt.insert(6);
    avt.insert(2);
    println!("{:?}", avt);
}

#[derive(Debug)]
struct AvlTree<T>
where
    T: Ord + Copy,
{
    data: Option<T>,
    left: Option<Box<AvlTree<T>>>,
    right: Option<Box<AvlTree<T>>>,
    factor: i16,
}

impl<T: Ord + Copy> AvlTree<T> {
    fn new(t: T) -> Self {
        AvlTree {
            data: Some(t),
            left: None,
            right: None,
            factor: 0,
        }
    }

    fn insert(&mut self, value: T) {
        match &self.data {
            Some(d) => {
                if value == *d {
                    self.data = Some(value);
                } else if value < *d {
                    match &mut self.left {
                        Some(l) => {
                            l.insert(value);
                        }
                        //left is none
                        None => {
                            self.left = Some(Box::new(AvlTree::new(value)));
                        }
                    }
                } else {
                    match &mut self.right {
                        Some(r) => {
                            r.insert(value);
                        }
                        None => {
                            self.right = Some(Box::new(AvlTree::new(value)));
                        }
                    }
                }
            }
            //this node is none
            None => self.data = Some(value),
        }
    }
}
