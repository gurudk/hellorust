use std::cmp::{max, Ordering::*};
use std::fmt::Debug;

fn main() {
    println!("Hello, world!");

    let mut bst = BinSearchTree::<&str, &str>::new();
    bst.insert("123", "abc");
    bst.insert("345", "asdfafaf");
    println!("{:?}", bst);

    println!("contains 123:{}", bst.contains(&"123"));
    println!("contains 345:{}", bst.contains(&"345"));
    println!("contains 333:{}", bst.contains(&"333"));

    println!("get(\"123\"):{:?}", bst.get(&"123"));

    println!("get(\"888\"):{:?}", bst.get(&"888"));
    bst.insert("888", "that is it!");

    println!("get(\"888\"):{:?}", bst.get(&"888"));

    bst.insert("92", "423442424");
    println!("{:?}", bst.max());
    println!("{}", "888" < "9");

    println!("the size of tree is :{}", bst.size());
    bst.insert("456", "kfal;f");
    println!("leaf size of tree is :{}", bst.leaf_size());

    // bst.preorder();

    let mut pre_iter = bst.preorder_iter();
    println!("{:?}", pre_iter.next());
    println!("{:?}", pre_iter.next());
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
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

    fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    fn contains(&self, key: &T) -> bool {
        match &self.key {
            Some(k) => {
                if *k == *key {
                    return true;
                } else {
                    if *key < *k {
                        match &self.left {
                            Some(ref l) => {
                                return l.contains(key);
                            }
                            None => {
                                return false;
                            }
                        }
                    } else {
                        match &self.right {
                            Some(ref r) => {
                                return r.contains(key);
                            }
                            None => {
                                return false;
                            }
                        }
                    }
                }
            }
            None => return false,
        }
    }

    fn size(&self) -> usize {
        let mut _size = 0;
        let root = self;

        _size = self.calc_size(root, _size);

        _size
    }

    fn leaf_size(&self) -> usize {
        let mut _size = 0;
        let root = self;
        _size = self.go_leaf(root, _size);

        _size
    }

    fn go_leaf(&self, bst: &BinSearchTree<T, V>, mut num: usize) -> usize {
        if bst.left.is_none() && bst.right.is_none() {
            num += 1;
        }

        match &bst.left {
            Some(l) => {
                num = self.go_leaf(l, num);
            }
            None => (),
        }

        match &bst.right {
            Some(r) => {
                num = self.go_leaf(r, num);
            }
            None => (),
        }

        num
    }

    fn calc_size(&self, bst: &BinSearchTree<T, V>, mut num: usize) -> usize {
        println!("begin num:{:?}", num);

        if !bst.key.is_none() {
            num += 1;
        }
        match &bst.left {
            Some(l) => {
                num = bst.calc_size(l, num);
                println!("type is:");
                print_type_of(&l);
            }
            None => (),
        };

        match &bst.right {
            Some(r) => {
                num = bst.calc_size(r, num);
            }
            None => (),
        }

        println!("after num:{:?}", num);

        num
    }

    fn max(&self) -> (Option<&T>, Option<&V>) {
        if self.is_empty() {
            return (None, None);
        } else {
            // print_type_of(&self);
            let mut r = self;
            // print_type_of(&self);
            while let Some(k) = &r.right {
                print_type_of(&k);
                r = k;
            }
            //as_ref() means Option<T> ----> Option<&T>
            return ((*r).key.as_ref(), r.value.as_ref());

            // match &self.right {
            //     Some(ref k) => {
            //         return k.max();
            //     }
            //     None => {
            //         return (self.key.as_ref(), self.value.as_ref());
            //     }
            // }
        }
    }

    fn get(&self, key: &T) -> Option<V> {
        match &self.key {
            Some(k) => {
                if *k == *key {
                    return self.value;
                } else {
                    if *key < *k {
                        match &self.left {
                            Some(ref l) => {
                                return l.get(key);
                            }
                            None => {
                                return None;
                            }
                        }
                    } else {
                        match &self.right {
                            Some(ref r) => {
                                return r.get(key);
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                }
            }
            None => return None,
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

    fn preorder_iter(&self) -> PreorderIterator<T, V> {
        let mut iter = PreorderIterator {
            queue: Box::new(Vec::new()),
        };

        let mut vec = Vec::new();

        let ret = self._preorder(Box::new(vec));

        iter.queue = ret;
        iter
    }

    fn _preorder<'a>(
        &'a self,
        mut vec: Box<Vec<(Option<&'a T>, Option<&'a V>)>>,
    ) -> Box<Vec<(Option<&'a T>, Option<&'a V>)>> {
        println!("before vec:{:?}", &vec);
        match &self.left {
            Some(l) => {
                vec = l._preorder(vec);
            }
            None => (),
        }

        vec.push((self.key.as_ref(), self.value.as_ref()));

        match &self.right {
            Some(r) => {
                vec = r._preorder(vec);
            }
            None => (),
        }

        println!("after vec:{:?}", &vec);
        vec
    }

    fn preorder(&self) {
        match &self.left {
            Some(l) => {
                l.preorder();
            }
            None => (),
        }
        println!("key:{:?},value:{:?}", self.key, self.value);

        match &self.right {
            Some(r) => {
                r.preorder();
            }
            None => (),
        }
    }
}

struct PreorderIterator<'a, T, V> {
    queue: Box<Vec<(Option<&'a T>, Option<&'a V>)>>,
}

impl<'a, T: 'a, V: 'a> Iterator for PreorderIterator<'a, T, V> {
    type Item = (Option<&'a T>, Option<&'a V>);
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.queue.remove(0))
    }
}
