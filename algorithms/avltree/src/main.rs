fn main() {
    println!("Hello, world!");

    let mut avl = AvlTree::new(5);
    avl.insert(3);
    avl.insert(10);
    avl.insert(6);
    avl.insert(13);
    avl.insert(1);
    avl.insert(4);
    avl.insert(18);
    avl.insert(20);

    println!("tree:{:?}", &avl);

    avl.levelorder();

    // if let Some(rcnode) = &avl.root {
    //     let root = rcnode.borrow();
    //     println!("left key:{:?}", &root.left_key());
    // }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct AvlTree<T>
where
    T: Copy + Ord,
{
    root: Option<Node<T>>,
}

#[derive(Debug)]
struct AvlNode<T> {
    key: Option<T>,
    left: Option<Node<T>>,
    right: Option<Node<T>>,
    parent: Option<WeakNode<T>>,
    factor: i32,
}

type Node<T> = Rc<RefCell<AvlNode<T>>>;
type WeakNode<T> = Weak<RefCell<AvlNode<T>>>;

impl<T: Ord + Copy + Debug> AvlTree<T> {
    fn new(key: T) -> Self {
        Self {
            root: Some(AvlNode::new_node(key)),
        }
    }

    fn insert(&mut self, key: T) {
        if let Some(root) = &self.root {
            let (is_left, deepened) = self.insert_at(key, root);
        } else {
            self.root = Some(AvlNode::new_node(key));
        }
    }

    fn insert_at(&self, key: T, atnode: &Node<T>) -> (bool, bool) {
        let mut node = atnode.borrow_mut();
        let mut ret = (false, false);
        if let Some(k) = node.key {
            if key == k {
                return ret;
            }

            if key < k {
                match &node.left {
                    None => {
                        //new left node
                        let new_node = AvlNode::new_node_with_parent(key, atnode);
                        node.left = Some(new_node);
                        node.factor += 1;

                        //如果是增加深度，则需要向上传播更新所有父节点的factor
                        if node.right.is_none() {
                            ret = (true, true);
                        } else {
                            ret = (true, false);
                        }
                    }
                    Some(lnode) => {
                        ret = self.insert_at(key, lnode);
                        let (is_left, deepened) = ret;
                        if deepened {
                            node.factor += 1;
                        }
                    }
                }
            } else {
                match &node.right {
                    None => {
                        let new_node = AvlNode::new_node_with_parent(key, atnode);
                        node.right = Some(new_node);
                        node.factor -= 1;

                        //如果是增加深度，则需要向上传播更新所有父节点的factor
                        if node.left.is_none() {
                            ret = (false, true);
                        } else {
                            ret = (false, false);
                        }
                    }
                    Some(rnode) => {
                        ret = self.insert_at(key, rnode);

                        let (is_left, deepened) = ret;
                        if deepened {
                            node.factor -= 1;
                        }
                    }
                }
            }
        } else {
            node.key = Some(key);
        }

        ret
    }

    // fn is_root(atnode:&Node<T>)->bool{
    //     let

    // }

    // fn update_parent_factor(&self, atnode: &Node<T>) {
    //     let node = atnode.borrow();
    //     let mut queue = VecDeque::new();
    //     if let Some(value) = &node.parent {
    //         queue.push_front(value.upgrade().unwrap());
    //     }

    //     while let Some(p_strong) = queue.pop_back() {
    //         let mut parent = p_strong.borrow_mut();
    //         if node.key == parent.left_key() {
    //             //node is left child,
    //             parent.factor += 1;
    //         } else if node.key == parent.right_key() {
    //             parent.factor -= 1;
    //         }

    //         if let Some(v) = &parent.parent {
    //             queue.push_front(v.upgrade().unwrap());
    //         }
    //     }
    // }

    fn levelorder(self) {
        let mut queue = VecDeque::new();
        if let Some(root) = &self.root {
            queue.push_front(Rc::clone(&root));
        }

        while !queue.is_empty() {
            if let Some(node) = &queue.pop_back() {
                let avlnode = node.borrow();
                println!("key={:?}, factor={:?}", avlnode.key, avlnode.factor);
                if let Some(lnode) = &avlnode.left {
                    queue.push_front(Rc::clone(&lnode));
                }

                if let Some(rnode) = &avlnode.right {
                    queue.push_front(Rc::clone(&rnode));
                }
            }
        }
    }
}

impl<T: Ord + Copy> AvlNode<T> {
    fn new(key: T) -> Self {
        Self {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
            factor: 0,
        }
    }

    fn new_node(key: T) -> Node<T> {
        Rc::new(RefCell::new(Self {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
            factor: 0,
        }))
    }

    fn new_node_with_parent(key: T, p: &Node<T>) -> Node<T> {
        Rc::new(RefCell::new(Self {
            key: Some(key),
            left: None,
            right: None,
            parent: Some(Rc::downgrade(p)),
            factor: 0,
        }))
    }

    fn new_with_parent(key: T, p: Node<T>) -> Self {
        Self {
            key: Some(key),
            left: None,
            right: None,
            parent: Some(Rc::downgrade(&p)),
            factor: 0,
        }
    }

    fn left_key(&self) -> Option<T> {
        if let Some(l) = &self.left {
            l.borrow().key
        } else {
            None
        }
    }

    fn right_key(&self) -> Option<T> {
        if let Some(r) = &self.right {
            r.borrow().key
        } else {
            None
        }
    }
}
