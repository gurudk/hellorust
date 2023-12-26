use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Tree {
    count: usize,
    root: Option<Rc<RefCell<Node>>>,
}


#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
}

impl Node{

    pub fn new(data:i32)->Rc<RefCell<Node>>{
        Rc::new(
            RefCell::new(
                Node{
                    data:data, 
                    left:None, 
                    right:None, 
                    parent:None
                }))
    }

    pub fn new_with_parent(data:i32, atnode:Rc<RefCell<Node>>)->Rc<RefCell<Node>>{
        Rc::new(
            RefCell::new(
                Node{
                    data:data,
                    left:None,
                    right:None,
                    parent:Some(Rc::downgrade(&atnode)),
                }
            )
        )
    }

}

impl Tree{
    /// Insert a new item into the tree; returns `true` if the insertion
    /// happened, and `false` if the given data was already present in the
    /// tree.
    pub fn insert(&mut self, data: i32) -> bool {
        if let Some(root) = self.root {
            if !self.insert_at(root, data) {
                return false;
            }
        } else {
            self.root = Some(Node::new(data));
        }
        self.count += 1;
        true
    }

    // Insert a new item into the subtree rooted at `atnode`.
    fn insert_at(&self, atnode: Rc<RefCell<Node>>, data: i32) -> bool {
        let mut node = atnode.borrow_mut();
        if data == node.data {
            false
        } else if data < node.data {
            match node.left {
                None => {
                    let new_node = Node::new_with_parent(data, atnode);
                    node.left = Some(new_node);
                    true
                }
                Some(lnode) => self.insert_at(lnode, data),
            }
        } else {
            match node.right {
                None => {
                    let new_node = Node::new_with_parent(data, atnode);
                    node.right = Some(new_node);
                    true
                }
                Some(rnode) => self.insert_at(rnode, data),
            }
        }
    }



}



fn main() {
    println!("Hello, world!");
}
