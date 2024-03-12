use std::cmp::{max, Ordering::*};
use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
}

struct BinSearchTree<T, V> {
    key: Option<T>,
    value: Option<V>,
    left: Option<Box<BinSearchTree<T, V>>>,
    right: Option<Box<BinSearchTree<T, V>>>,
}

impl<T, V> BinSearchTree<T, V> {
    fn new() -> Self {
        Self {
            key: None,
            value: None,
            left: None,
            right: None,
        }
    }
}
