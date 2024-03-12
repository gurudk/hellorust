use std::cmp::{max, Ordering::*};
use std::fmt::Debug;

fn main() {
    println!("Hello, world!");

    let mut bst = BinSearchTree::<&str, &str>::new();
    bst.insert("123", "abc");
    bst.insert("345", "asdfafaf");
    println!("{:?}", bst);
}

#[derive(Debug)]
struct BinSearchTree<T, V> {
    key: Option<T>,
    value: Option<V>,
    left: Option<Box<BinSearchTree<T, V>>>,
    right: Option<Box<BinSearchTree<T, V>>>,
}

impl<T, V> BinSearchTree<T, V>
where
    T: Copy + Ord + Debug,
    V: Copy + Debug,
{
    fn new() -> Self {
        Self {
            key: None,
            value: None,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, key: T, val: V) {
        if self.key.is_none() {
            self.key = Some(key);
            self.value = Some(val);
        } else {
            match &self.key {
                Some(k) => {
                    if key == *k {
                        self.value = Some(val);
                    }

                    let child = if key < *k {
                        &mut self.left
                    } else {
                        &mut self.right
                    };

                    match child {
                        Some(ref mut node) => {
                            node.insert(key, val);
                        }
                        None => {
                            let mut node = BinSearchTree::new();
                            node.insert(key, val);
                            *child = Some(Box::new(node));
                        }
                    }
                }
                None => (),
            }
        }
    }
}
